use atomic_wait;
use std::cell::UnsafeCell;
use std::ops::{Deref, DerefMut};
use std::sync::atomic::{
    AtomicU32,
    Ordering::{Acquire, Relaxed, Release},
};

pub struct RwLock<T> {
    // リードロックの数を２倍し、ライタが待機してた1足した値
    // ライトロックされていたら u32::MAX
    //
    // したがって、リーダは`state`が奇数ならロックを取得可能
    // 奇数ならブロックする
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
            // 偶数
            if s % 2 == 0 {
                assert!(s < u32::MAX - 2, "too many read");
                match self.state.compare_exchange_weak(s, s + 2, Acquire, Relaxed) {
                    Ok(_) => return ReadGuard { rwlock: self },
                    Err(e) => s = e,
                }
            }
            // 奇数
            if s % 2 == 1 {
                atomic_wait::wait(&self.state, s);
                s = self.state.load(Relaxed);
            }
        }
    }

    pub fn write(&self) -> WriteGuard<T> {
        let mut s = self.state.load(Relaxed);
        loop {
            // アンロックされたあらロックを試みる
            if s <= 1 {
                match self.state.compare_exchange(s, u32::MAX, Acquire, Relaxed) {
                    Ok(_) => return WriteGuard { rwlock: self },
                    Err(e) => {
                        s = e;
                        continue;
                    }
                }
            }
            // stateを奇数にして、新しいリーダをブロックする
            if s % 2 == 0 {
                match self.state.compare_exchange(s, s + 1, Relaxed, Relaxed) {
                    Ok(_) => {}
                    Err(e) => {
                        s = e;
                        continue;
                    }
                }
            }
            // まだロックされていたら待機
            let w = self.writer_wake_counter.load(Acquire);
            s = self.state.load(Relaxed);
            if s >= 2 {
                atomic_wait::wait(&self.writer_wake_counter, w);
                s = self.state.load(Relaxed);
            }
        }
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
        // stateを2減らして、1つリードロックを削除する
        if self.rwlock.state.fetch_sub(2, Release) == 3 {
            // 3から1になった場合はRwLock<T>がアンロックされ、
            // かつ待機中のライタがいることがわかる
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
