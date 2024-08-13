use std::ops::Deref;
use std::ptr::NonNull;
use std::sync::atomic::{AtomicUsize, Ordering::Relaxed};

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
