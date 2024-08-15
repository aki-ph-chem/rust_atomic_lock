use std::cell::UnsafeCell;
use std::mem::ManuallyDrop;
use std::ops::Deref;
use std::ptr::NonNull;
use std::sync::atomic::{
    fence, AtomicUsize,
    Ordering::{Acquire, Relaxed, Release},
};

pub struct Arc<T> {
    ptr: NonNull<ArcData<T>>,
}

unsafe impl<T> Send for Arc<T> where T: Sync + Send {}
unsafe impl<T> Sync for Arc<T> where T: Sync + Send {}

impl<T> Arc<T> {
    pub fn new(data: T) -> Self {
        Self {
            ptr: NonNull::from(Box::leak(Box::new(ArcData {
                alloc_ref_count: AtomicUsize::new(1),
                data_ref_count: AtomicUsize::new(1),
                data: UnsafeCell::new(ManuallyDrop::new(data)),
            }))),
        }
    }

    fn data(&self) -> &ArcData<T> {
        unsafe { self.ptr.as_ref() }
    }

    pub fn get_mut(arc: &mut Self) -> Option<&mut T> {
        // Acquire はWeak::dropのReleaseデクリメントに対応する
        // アップグレードされたWeakであれば、次のdat_ref_count.load()
        // で観測できるようにするため
        if arc
            .data()
            .alloc_ref_count
            .compare_exchange(1, usize::MAX, Acquire, Relaxed)
            .is_err()
        {
            return None;
        }
        let is_unique = arc.data().data_ref_count.load(Relaxed) == 1;
        // Releaseは`downgrade()`のAcquireインクリメントに対応
        // `downgrade()`以降のdata_ref_countへの何からの変更が
        // 上のis_uniqueの結果に影響しないようにするため
        arc.data().alloc_ref_count.store(1, Release);
        if !is_unique {
            return None;
        }

        // AcquireはArc::dropのReleaseデクリメントに対応
        // 他の何もデータにアクセスしていないことを保証するため
        fence(Acquire);
        unsafe { Some(&mut *arc.data().data.get()) }
    }

    pub fn downgrade(arc: &Self) -> Weak<T> {
        let mut n = arc.data().alloc_ref_count.load(Relaxed);
        loop {
            if n == usize::MAX {
                std::hint::spin_loop();
                n = arc.data().alloc_ref_count.load(Relaxed);
                continue;
            }
            assert!(n < usize::MAX - 1);
            // Acquireはget_mut()のReleaseストアと同期
            if let Err(e) =
                arc.data()
                    .alloc_ref_count
                    .compare_exchange_weak(n, n + 1, Acquire, Relaxed)
            {
                n = e;
                continue;
            }
            return Weak { ptr: arc.ptr };
        }
    }
}

impl<T> Deref for Arc<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        // 安全性: このデータに対するArcが存在するので、
        // データは存在し、共有されている可能性もある
        unsafe { &*self.data().data.get() }
    }
}

impl<T> Clone for Arc<T> {
    fn clone(&self) -> Self {
        if self.data().data_ref_count.fetch_add(1, Relaxed) > usize::MAX / 2 {
            std::process::abort();
        }

        Self { ptr: self.ptr }
    }
}

impl<T> Drop for Arc<T> {
    fn drop(&mut self) {
        if self.data().data_ref_count.fetch_sub(1, Release) == 1 {
            fence(Acquire);
            // 安全性: データへの参照カウントは0なので
            // だれもアクセスしていない
            unsafe {
                ManuallyDrop::drop(&mut *self.data().data.get());
            }
            // `Arc<T>`が残っていないので、全ての`Arc<T>`を代表していた
            // 暗黙のWeak<T>をdropする
            drop(Weak { ptr: self.ptr })
        }
    }
}

pub struct Weak<T> {
    ptr: NonNull<ArcData<T>>,
}

unsafe impl<T> Send for Weak<T> where T: Send + Sync {}
unsafe impl<T> Sync for Weak<T> where T: Send + Sync {}

impl<T> Weak<T> {
    fn data(&self) -> &ArcData<T> {
        unsafe { self.ptr.as_ref() }
    }

    /// Weak<T>からArc<T>に変換する
    pub fn upgrade(&self) -> Option<Arc<T>> {
        let mut n = self.data().data_ref_count.load(Relaxed);
        loop {
            if n == 0 {
                return None;
            }
            assert!(n < usize::MAX);
            if let Err(e) =
                self.data()
                    .data_ref_count
                    .compare_exchange_weak(n, n + 1, Relaxed, Relaxed)
            {
                n = e;
                continue;
            }

            return Some(Arc { ptr: self.ptr });
        }
    }
}

impl<T> Clone for Weak<T> {
    fn clone(&self) -> Self {
        if self.data().alloc_ref_count.fetch_add(1, Relaxed) > usize::MAX / 2 {
            std::process::abort();
        }

        Weak { ptr: self.ptr }
    }
}

impl<T> Drop for Weak<T> {
    fn drop(&mut self) {
        if self.data().alloc_ref_count.fetch_sub(1, Release) == 1 {
            fence(Acquire);
            unsafe {
                drop(Box::from_raw(self.ptr.as_ptr()));
            }
        }
    }
}

struct ArcData<T> {
    /// `Arc`の数
    data_ref_count: AtomicUsize,
    /// `Weak`の数`Arc`が一個でもあったらさらに一個足す
    alloc_ref_count: AtomicUsize,
    /// データ: Wekaしかなくなったらdropされる
    data: UnsafeCell<ManuallyDrop<T>>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::thread;

    #[test]
    fn test_arc() {
        static NUM_DROPS: AtomicUsize = AtomicUsize::new(0);

        struct DetectDrop;
        impl Drop for DetectDrop {
            fn drop(&mut self) {
                NUM_DROPS.fetch_add(1, Relaxed);
            }
        }

        // この時点ではWeakはアップグレード可能
        let x = Arc::new(("Hello", DetectDrop));
        let y = Arc::downgrade(&x);
        let z = Arc::downgrade(&x);

        let t = thread::spawn(move || {
            let y = y.upgrade().unwrap();
            //assert_eq!(y.0, "Hello");
        });
        assert_eq!(x.0, "Hello");
        t.join().unwrap();

        // データはまだdropされていないはずなので、Weakはアップグレード可能
        assert_eq!(NUM_DROPS.load(Relaxed), 0);
        assert!(z.upgrade().is_some());

        drop(x);

        // データはdropされているので、Weakはアップグレード不可能
        assert_eq!(NUM_DROPS.load(Relaxed), 1);
        assert!(z.upgrade().is_none());
    }
}
