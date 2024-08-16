use rust_atomic_lock::lock::{mutex, mutex_opt, mutex_spin};
use std::hint::black_box;
use std::thread;
use std::time;

fn main() {
    let num_iter = 5_000_000;

    // mutex: 2状態
    let m = mutex::Mutex::new(0);
    black_box(&m);
    let start = time::Instant::now();
    thread::scope(|s| {
        for _ in 0..4 {
            s.spawn(|| {
                for _ in 0..num_iter {
                    *m.lock() += 1;
                }
            });
        }
    });
    let duration = start.elapsed();
    println!("mutex: loked {} times in {:?}", *m.lock(), duration);

    // mutex_opt: 3状態
    let m = mutex_opt::Mutex::new(0);
    black_box(&m);
    let start = time::Instant::now();
    thread::scope(|s| {
        for _ in 0..4 {
            s.spawn(|| {
                for _ in 0..num_iter {
                    *m.lock() += 1;
                }
            });
        }
    });
    let duration = start.elapsed();
    println!("mutex_opt: loked {} times in {:?}", *m.lock(), duration);

    // mutex_opt: 3状態 + スピンロック
    let m = mutex_spin::Mutex::new(0);
    black_box(&m);
    let start = time::Instant::now();
    thread::scope(|s| {
        for _ in 0..4 {
            s.spawn(|| {
                for _ in 0..num_iter {
                    *m.lock() += 1;
                }
            });
        }
    });
    let duration = start.elapsed();
    println!("mutex_spin: loked {} times in {:?}", *m.lock(), duration);
}
