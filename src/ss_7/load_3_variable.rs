use std::hint::black_box;
use std::sync::atomic::{AtomicU64, Ordering::Relaxed};
use std::thread;
use std::time::Instant;

static A: [AtomicU64; 3] = [AtomicU64::new(0), AtomicU64::new(0), AtomicU64::new(0)];

fn load_3_variable() {
    let num_iter = 1_000_000_000;
    for _ in 0..num_iter {
        black_box(A[1].load(Relaxed));
    }
}

fn main() {
    black_box(&A);
    thread::spawn(|| loop {
        A[0].store(0, Relaxed);
        A[2].store(0, Relaxed);
    });

    let start = Instant::now();
    load_3_variable();
    // intel core i7-13700で約 600 ms
    println!("{:?}", start.elapsed());
}
