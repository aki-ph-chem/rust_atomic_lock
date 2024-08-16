use std::hint::black_box;
use std::sync::atomic::{AtomicU64, Ordering::Relaxed};
use std::thread;
use std::time::Instant;

static A: AtomicU64 = AtomicU64::new(0);

// 関数をブラックボックス化して最適化でループが削られないようにする
fn load_billion_times_black_box() {
    let num_iter = 1_000_000_000;
    for _ in 0..num_iter {
        black_box(A.load(Relaxed));
    }
}

fn main() {
    black_box(&A);

    // 別スレッドからAをロード
    thread::spawn(|| loop {
        // load()のときよりも約3倍遅い
        black_box(A.store(0, Relaxed));
    });

    let start = Instant::now();
    load_billion_times_black_box();
    // intel core i7-13700で約 600 ms
    println!("{:?}", start.elapsed());
}
