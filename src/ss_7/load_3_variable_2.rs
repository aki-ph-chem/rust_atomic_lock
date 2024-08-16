use std::hint::black_box;
use std::sync::atomic::{AtomicU64, Ordering::Relaxed};
use std::thread;
use std::time::Instant;

// 64バイトでアラインして、
// 一つ一つが別のキャッシュラインに乗るようにする
#[repr(align(64))]
struct Aligned(AtomicU64);

static A_ALIGNED: [Aligned; 3] = [
    Aligned(AtomicU64::new(0)),
    Aligned(AtomicU64::new(0)),
    Aligned(AtomicU64::new(0)),
];

fn load_3_variable_aligned() {
    let num_iter = 1_000_000_000;
    for _ in 0..num_iter {
        black_box(A_ALIGNED[1].0.load(Relaxed));
    }
}

fn main() {
    black_box(&A_ALIGNED);
    thread::spawn(|| loop {
        A_ALIGNED[0].0.store(0, Relaxed);
        A_ALIGNED[2].0.store(0, Relaxed);
    });

    let start = Instant::now();
    load_3_variable_aligned();
    // intel core i7-13700で約 200 ms
    println!("{:?}", start.elapsed());
}
