use std::sync::atomic::{AtomicI32, Ordering::Relaxed};

fn main() {
    let a = AtomicI32::new(100);
    // bは古いaの値(100)
    let b = a.fetch_add(23, Relaxed);
    // aの値を読み込んで返す
    let c = a.load(Relaxed);

    assert_eq!(b, 100);
    assert_eq!(c, 123);
}
