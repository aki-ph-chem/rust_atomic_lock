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

/// 値を返さない(addl命令)
#[no_mangle]
pub fn add_ten(x: &AtomicI32) {
    x.fetch_add(10, Relaxed);
}

/// 値を返す(xaddl命令)
#[no_mangle]
pub fn add_ten_i32(x: &AtomicI32) -> i32 {
    x.fetch_add(10, Relaxed)
}

// 比較交換命令

/// fetch_or()を使って実装
#[no_mangle]
pub fn cmp_ten_exchg(x: &AtomicI32) -> i32 {
    x.fetch_or(10, Relaxed)
}

/// compare_exchange()を使って実装
/// 上と等価
#[no_mangle]
pub fn cmp_ten_exchg_loop(x: &AtomicI32) -> i32 {
    let mut current = x.load(Relaxed);
    loop {
        let new = current | 10;
        match x.compare_exchange(current, new, Relaxed, Relaxed) {
            Ok(v) => return v,
            Err(v) => current = v,
        }
    }
}
