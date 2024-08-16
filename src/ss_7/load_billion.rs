use std::hint::black_box;
use std::sync::atomic::{AtomicU64, Ordering::Relaxed};
use std::time::Instant;

static A: AtomicU64 = AtomicU64::new(0);

// --releaseで実行するとループが削られる
fn load_billion_times() {
    let num_iter = 1_000_000_000;
    for _ in 0..num_iter {
        A.load(Relaxed);
    }
}

// 関数をブラックボックス化して最適化でループが削られないようにする
fn load_billion_times_black_box() {
    let num_iter = 1_000_000_000;
    for _ in 0..num_iter {
        black_box(A.load(Relaxed));
    }
}

fn main() {
    let start = Instant::now();
    load_billion_times();
    println!("load_billion_times(): {:?}", start.elapsed());

    let start = Instant::now();
    load_billion_times_black_box();
    println!("load_billion_times_black_box(): {:?}", start.elapsed());
}
