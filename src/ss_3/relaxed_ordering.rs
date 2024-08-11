use std::sync::atomic::{AtomicI32, Ordering::Relaxed};
use std::thread;

static X: AtomicI32 = AtomicI32::new(0);

fn a() {
    X.fetch_add(5, Relaxed);
    X.fetch_add(10, Relaxed);
}

fn a_1() {
    X.fetch_add(5, Relaxed);
}

fn a_2() {
    X.fetch_add(10, Relaxed);
}

fn b() {
    let a = X.load(Relaxed);
    let b = X.load(Relaxed);
    let c = X.load(Relaxed);
    let d = X.load(Relaxed);
    println!("a, b, c, d: {a}, {b}, {c}, {d}");
}

fn main() {
    thread::scope(|s| {
        //s.spawn(|| a());
        s.spawn(|| a_1());
        s.spawn(|| a_2());
        s.spawn(|| b());
    });
}
