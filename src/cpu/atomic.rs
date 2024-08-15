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

// リード・モディファイ

// 値を返さない(addl命令)
#[no_mangle]
pub fn add_ten(x: &AtomicI32) {
    x.fetch_add(10, Relaxed);
}

// 値を返す(xaddl命令)
#[no_mangle]
pub fn add_ten_i32(x: &AtomicI32) -> i32 {
    x.fetch_add(10, Relaxed)
}
