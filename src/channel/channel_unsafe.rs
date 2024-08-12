use std::cell::UnsafeCell;
use std::mem::MaybeUninit;
use std::sync::atomic::{
    AtomicBool,
    Ordering::{Acquire, Release},
};

pub struct Channel<T> {
    message: UnsafeCell<MaybeUninit<T>>,
    ready: AtomicBool,
}

unsafe impl<T> Sync for Channel<T> where T: Send {}

impl<T> Channel<T> {
    pub fn new() -> Self {
        Self {
            message: UnsafeCell::new(MaybeUninit::uninit()), // チャネルの内容は空で初期化
            ready: AtomicBool::new(false),
        }
    }

    /// 一度しか読んではならない
    pub unsafe fn send(&self, message: T) {
        (*self.message.get()).write(message);
        self.ready.store(true, Release);
    }

    pub fn is_ready(&self) -> bool {
        self.ready.load(Acquire)
    }

    /// `is_read`がtrueを返す場合にのみ呼ぶこと
    ///
    /// 一度しか読んではならない
    pub unsafe fn receive(&self) -> T {
        (*self.message.get()).assume_init_read()
    }
}
