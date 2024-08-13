use std::cell::UnsafeCell;
use std::mem::MaybeUninit;
use std::sync::atomic::{
    AtomicBool,
    Ordering::{Acquire, Relaxed, Release},
};

pub struct Channel<T> {
    message: UnsafeCell<MaybeUninit<T>>,
    in_use: AtomicBool,
    ready: AtomicBool,
}

unsafe impl<T> Sync for Channel<T> where T: Send {}

impl<T> Channel<T> {
    pub fn new() -> Self {
        Self {
            message: UnsafeCell::new(MaybeUninit::uninit()), // チャネルの内容は空で初期化
            in_use: AtomicBool::new(false),
            ready: AtomicBool::new(false),
        }
    }

    /// 二つ以上のメッセージを送信しようとしたらpanic
    pub fn send(&self, message: T) {
        if self.in_use.swap(true, Relaxed) {
            panic!("can't send more than one message");
        }

        unsafe { (*self.message.get()).write(message) };
        self.ready.store(true, Release);
    }

    pub fn is_ready(&self) -> bool {
        self.ready.load(Relaxed)
    }

    /// メッセージが空ならpanic
    /// メッセージがすでに読み込まれていてもpanic
    ///
    /// `is_read`がtrueを返す場合にのみ呼ぶこと
    pub fn receive(&self) -> T {
        if !self.ready.swap(true, Acquire) {
            panic!("no message available");
        }

        // 安全性: readyをチェックしてフラッグをリセットした
        unsafe { (*self.message.get()).assume_init_read() }
    }
}

// Drop traitの実装
impl<T> Drop for Channel<T> {
    fn drop(&mut self) {
        // アトミック操作を用いていない
        if *self.ready.get_mut() {
            unsafe { self.message.get_mut().assume_init_drop() }
        }
    }
}
