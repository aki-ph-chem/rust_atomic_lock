use atomic_wait;
use std::cell::UnsafeCell;
use std::ops::{Deref, DerefMut};
use std::sync::atomic::{
    AtomicU32,
    Ordering::{Acquire, Release},
};

pub struct Mutex<T> {
    /// 0: unlocked
    /// 1: locked
    state: AtomicU32,
    /// 値
    value: UnsafeCell<T>,
}

unsafe impl<T> Sync for Mutex<T> where T: Send {}

impl<T> Mutex<T> {
    pub const fn new(value: T) -> Self {
        Self {
            state: AtomicU32::new(0), // unlocked状態
            value: UnsafeCell::new(value),
        }
    }

    /// lockする
    pub fn lock(&self) -> MutexGuard<T> {
        // stateを1にセットする(lockする)
        while self.state.swap(1, Acquire) == 1 {
            // すでにロックされていた場合、
            // stateが1でなくなるまでwaitする
            atomic_wait::wait(&self.state, 1);
        }

        MutexGuard { mutex: self }
    }
}

pub struct MutexGuard<'a, T> {
    mutex: &'a Mutex<T>,
}

unsafe impl<T> Sync for MutexGuard<'_, T> where T: Sync {}

impl<T> Deref for MutexGuard<'_, T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.mutex.value.get() }
    }
}

impl<T> DerefMut for MutexGuard<'_, T> {
    fn deref_mut(&mut self) -> &mut T {
        unsafe { &mut *self.mutex.value.get() }
    }
}

impl<T> Drop for MutexGuard<'_, T> {
    fn drop(&mut self) {
        // stateを0(unlock)に戻す
        self.mutex.state.store(0, Release);
        // 待機中のスレッドがあればその一つを起こす
        atomic_wait::wake_one(&self.mutex.state);
    }
}
