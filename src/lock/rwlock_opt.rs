use atomic_wait;
use std::cell::UnsafeCell;
use std::ops::{Deref, DerefMut};
use std::sync::atomic::{
    AtomicU32,
    Ordering::{Acquire, Relaxed, Release},
};

pub struct RwLock<T> {
    // リーダの数
    state: AtomicU32,
    // ライタを起こす際にインクリメントする
    writer_wake_counter: AtomicU32,
    // 値
    value: UnsafeCell<T>,
}

unsafe impl<T> Sync for RwLock<T> where T: Send + Sync {}

impl<T> RwLock<T> {
    pub fn new(value: T) -> Self {
        Self {
            state: AtomicU32::new(0),
            writer_wake_counter: AtomicU32::new(0),
            value: UnsafeCell::new(value),
        }
    }

    pub fn read(&self) -> ReadGuard<T> {
        let mut s = self.state.load(Relaxed);
        loop {
            if s < u32::MAX {
                assert!(s != u32::MAX - 1, "too many read");
                match self.state.compare_exchange_weak(s, s + 1, Acquire, Relaxed) {
                    Ok(_) => return ReadGuard { rwlock: self },
                    Err(e) => s = e,
                }
            }
            if s == u32::MAX {
                atomic_wait::wait(&self.state, u32::MAX);
                s = self.state.load(Relaxed);
            }
        }
    }

    pub fn write(&self) -> WriteGuard<T> {
        while self
            .state
            .compare_exchange(0, u32::MAX, Acquire, Relaxed)
            .is_err()
        {
            let w = self.writer_wake_counter.load(Acquire);
            if self.state.load(Relaxed) != 0 {
                // RwLockがまだlockされていたら待機
                // ただし、チェックした後でウェイク通知が来てない場合のみ
                atomic_wait::wait(&self.write_wake_counter, w);
            }
        }

        WriteGuard { rwlock: self }
    }
}

pub struct ReadGuard<'a, T> {
    rwlock: &'a RwLock<T>,
}

// ReadGuardは共有参照(&T)のように振る舞うことを要求する
// そのためにDerefのみを実装する

impl<T> Deref for ReadGuard<'_, T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.rwlock.value.get() }
    }
}

impl<T> Drop for ReadGuard<'_, T> {
    fn drop(&mut self) {
        if self.rwlock.state.fetch_sub(1, Release) == 1 {
            self.rwlock.writer_wake_counter.fetch_add(1, Release);
            // 待機中のライタがあればそれを起こす
            atomic_wait::wake_one(&self.rwlock.writer_wake_counter);
        }
    }
}

pub struct WriteGuard<'a, T> {
    rwlock: &'a RwLock<T>,
}

// WriteGuardは排他参照(&mut T)のように振る舞うことを要求する
// そのためにDeref,DerefMutを両方実装する

impl<T> Deref for WriteGuard<'_, T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.rwlock.value.get() }
    }
}

impl<T> DerefMut for WriteGuard<'_, T> {
    fn deref_mut(&mut self) -> &mut T {
        unsafe { &mut *self.rwlock.value.get() }
    }
}

impl<T> Drop for WriteGuard<'_, T> {
    fn drop(&mut self) {
        self.rwlock.state.store(0, Release);
        self.rwlock.writer_wake_counter.fetch_add(1, Release);
        atomic_wait::wake_one(&self.rwlock.writer_wake_counter);
        // 待機しているリーダとライタを全て起こす
        atomic_wait::wake_all(&self.rwlock.state);
    }
}
