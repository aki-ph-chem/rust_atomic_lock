use std::cell::UnsafeCell;
use std::ops::{Deref, DerefMut};
use std::sync::atomic::{
    AtomicBool,
    Ordering::{Acquire, Release},
};

/// unsafeによるスピンロックの実装
pub struct SpinLock<T> {
    locked: AtomicBool,
    value: UnsafeCell<T>,
}

impl<T> SpinLock<T> {
    pub fn new(value: T) -> Self {
        Self {
            locked: AtomicBool::new(false),
            value: UnsafeCell::new(value),
        }
    }

    pub fn lock(&self) -> Guard<T> {
        while self.locked.swap(true, Acquire) {
            std::hint::spin_loop();
        }

        Guard { lock: self }
    }

    pub unsafe fn unlock(&self) {
        self.locked.store(false, Release);
    }
}

// Sendを実装する全ての型TにSpinLock<T>にSyncを実装する
unsafe impl<T> Sync for SpinLock<T> where T: Send {}

/// &mut Tをラップしたガード
pub struct Guard<'a, T> {
    lock: &'a SpinLock<T>,
}

impl<T> Deref for Guard<'_, T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        // このガードの存在自体が、
        // ロックを排他的に取得したことを意味する
        unsafe { &*self.lock.value.get() }
    }
}

impl<T> DerefMut for Guard<'_, T> {
    fn deref_mut(&mut self) -> &mut T {
        // このガードの存在自体が、
        // ロックを排他的に取得したことを意味する
        unsafe { &mut *self.lock.value.get() }
    }
}

// Send, Syncを実装
unsafe impl<T> Send for Guard<'_, T> where T: Send {}
unsafe impl<T> Sync for Guard<'_, T> where T: Sync {}

// Dropを実装
impl<T> Drop for Guard<'_, T> {
    fn drop(&mut self) {
        self.lock.locked.store(false, Release);
    }
}
