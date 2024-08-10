use std::sync::atomic::{AtomicI32, Ordering::Relaxed};
use std::thread;

static X: AtomicI32 = AtomicI32::new(0);
static Y: AtomicI32 = AtomicI32::new(0);

fn a() {
    X.store(10, Relaxed); // (1)
    Y.store(20, Relaxed); // (2)
}

fn b() {
    let y = Y.load(Relaxed); // (3)
    let x = X.load(Relaxed); // (4)

    println!("x, y: {x}, {y}")
}

fn main() {
    thread::scope(|s| {
        // スレッド1
        s.spawn(|| a());
        // スレッド2
        s.spawn(|| b());
    });
}
