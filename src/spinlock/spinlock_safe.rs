use std::sync::atomic::{
    AtomicBool,
    Ordering::{Acquire, Release},
};

/// 最小限のスピンロック
pub struct SpinLock {
    locked: AtomicBool,
}

impl SpinLock {
    pub const fn new() -> Self {
        Self {
            locked: AtomicBool::new(false),
        }
    }

    pub fn lock(&self) {
        // lockedをtrueに変更
        while self.locked.swap(true, Acquire) {
            std::hint::spin_loop(); // spinしてロックされるのを待機することを伝える
        }
    }

    pub fn unlock(&self) {
        self.locked.store(false, Release);
    }
}
