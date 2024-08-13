// より省メモリな実装(1バイト少なく実装)
use std::cell::UnsafeCell;
use std::mem::MaybeUninit;
use std::sync::atomic::{
    AtomicU8,
    Ordering::{Acquire, Relaxed, Release},
};

const EMTPY: u8 = 0;
const WRITING: u8 = 1;
const READY: u8 = 2;
const READING: u8 = 3;

pub struct Channel<T> {
    message: UnsafeCell<MaybeUninit<T>>,
    /// 状態をAtomicU8一つで管理
    state: AtomicU8,
}

unsafe impl<T> Sync for Channel<T> where T: Send {}

impl<T> Channel<T> {
    pub const fn new() -> Self {
        Self {
            message: UnsafeCell::new(MaybeUninit::uninit()),
            state: AtomicU8::new(EMTPY),
        }
    }

    pub fn send(&self, message: T) {
        if self
            .state
            .compare_exchange(EMTPY, WRITING, Relaxed, Relaxed)
            .is_err()
        {
            panic!("Can't send more than message!");
        }
        unsafe { (*self.message.get()).write(message) };
        self.state.store(READY, Release);
    }

    pub fn is_ready(&self) -> bool {
        self.state.load(Relaxed) == READY
    }

    pub fn receive(&self) -> T {
        if self
            .state
            .compare_exchange(READY, READING, Acquire, Relaxed)
            .is_err()
        {
            panic!("no message available!");
        }
        unsafe { (*self.message.get()).assume_init_read() }
    }
}

impl<T> Drop for Channel<T> {
    fn drop(&mut self) {
        if *self.state.get_mut() == READY {
            unsafe { self.message.get_mut().assume_init_drop() }
        }
    }
}
