use atomic_wait;
use std::cell::UnsafeCell;
use std::ops::{Deref, DerefMut};
use std::sync::atomic::{
    AtomicU32,
    Ordering::{Acquire, Relaxed, Release},
};

pub struct Mutex<T> {
    /// 0: unlocked
    /// 1: locked 他に待機スレッドはない
    /// 2: locked 他に待機スレッドがある
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
    /// スピンロックを使う
    pub fn lock(&self) -> MutexGuard<T> {
        // まず比較交換操作で0->1に変更を試みる
        if self.state.compare_exchange(0, 1, Acquire, Relaxed).is_err() {
            // このロックはすでにロックされている
            Self::lock_contented(&self.state);
        }

        MutexGuard { mutex: self }
    }

    fn lock_contented(state: &AtomicU32) {
        let mut spin_count = 0;

        // まず100回スピンする
        while state.load(Relaxed) == 1 && spin_count < 100 {
            spin_count += 1;
            std::hint::spin_loop();
        }

        // スピンを抜けたらロックを試みる
        if state.compare_exchange(0, 1, Acquire, Relaxed).is_ok() {
            return;
        }

        while state.swap(2, Acquire) != 0 {
            atomic_wait::wait(state, 2);
        }
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
        // もとの値が2の場合のみスレッドを起こす
        if self.mutex.state.swap(0, Release) == 2 {
            atomic_wait::wake_one(&self.mutex.state);
        }
    }
}
