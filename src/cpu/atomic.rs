use std::sync::atomic::{AtomicI32, Ordering::Relaxed};

// ロード

#[no_mangle]
pub fn store(x: &AtomicI32) {
    x.store(0, Relaxed);
}

#[no_mangle]
pub fn load(x: &AtomicI32) -> i32 {
    x.load(Relaxed)
}
