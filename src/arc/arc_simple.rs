use std::ops::Deref;
use std::ptr::NonNull;
use std::sync::atomic::{
    fence, AtomicUsize,
    Ordering::{Acquire, Relaxed, Release},
};

/// データの本体,private
struct ArcData<T> {
    ref_count: AtomicUsize,
    data: T,
}

/// Arc: public
pub struct Arc<T> {
    ptr: NonNull<ArcData<T>>,
}

// Tに対してSend,Syncを実装する
unsafe impl<T> Send for Arc<T> where T: Send + Sync {}
unsafe impl<T> Sync for Arc<T> where T: Send + Sync {}

impl<T> Arc<T> {
    pub fn new(data: T) -> Self {
        Self {
            ptr: NonNull::from(Box::leak(Box::new(ArcData {
                ref_count: AtomicUsize::new(1),
                data,
            }))),
        }
    }

    /// ArcDataを扱うためのヘルパー関数
    fn data(&self) -> &ArcData<T> {
        unsafe { self.ptr.as_ref() }
    }
}

// 共有参照のみ実装(排他参照は実装しない)
impl<T> Deref for Arc<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &self.data().data
    }
}

impl<T> Clone for Arc<T> {
    /// 参照カウンタをインクリメントして、実態へのポインタを返す
    fn clone(&self) -> Self {
        // 参照カウントが、usizeの最大値の半分の値になったら異常終了
        if self.data().ref_count.fetch_add(1, Relaxed) > usize::MAX / 2 {
            std::process::abort();
        }

        self.data().ref_count.fetch_add(1, Relaxed);
        Arc { ptr: self.ptr }
    }
}

impl<T> Drop for Arc<T> {
    fn drop(&mut self) {
        // ToDo: メモリーオーダリング(とりあえずRlexedを入れておいた)
        if self.data().ref_count.fetch_sub(1, Release) == 1 {
            fence(Acquire);
            unsafe {
                drop(Box::from_raw(self.ptr.as_ptr()));
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::thread;

    #[test]
    fn test_arc() {
        static NUM_DROPS: AtomicUsize = AtomicUsize::new(0);

        struct DtectDrop;
        impl Drop for DtectDrop {
            fn drop(&mut self) {
                NUM_DROPS.fetch_add(1, Relaxed);
            }
        }

        // 文字列とDetectDropを保持するオブジェクトを共有
        // Arcを二個作る。DetectDropでいつdropされたかがわかる
        let x = Arc::new(("hello", DtectDrop));
        let y = x.clone();

        // xをもう一つのスレッドに送り、そこで使う
        let t = thread::spawn(move || {
            assert_eq!(x.0, "hello");
        });

        // yは使える
        assert_eq!(y.0, "hello");

        // スレッドtの終了を待機
        t.join().unwrap();

        // xはここでdropされているはず
        // が、yが参照しているのでオブジェクトはまだ生きてる
        assert_eq!(NUM_DROPS.load(Relaxed), 0);

        // 残ったyをdrop
        drop(y);

        // yもdropされたので、オブジェクトもdropされたはず
        assert_eq!(NUM_DROPS.load(Relaxed), 1);
    }
}
