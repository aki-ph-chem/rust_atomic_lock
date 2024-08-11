use std::sync::atomic::{
    AtomicBool, AtomicU64,
    Ordering::{Acquire, Relaxed, Release},
};
use std::thread;
use std::time::Duration;

static DATA: AtomicU64 = AtomicU64::new(0);
static READY: AtomicBool = AtomicBool::new(false);

// Release: ストア操作に適用
// Acquire: ロード操作に適用

fn main() {
    thread::spawn(|| {
        DATA.store(123, Relaxed);
        thread::sleep(Duration::from_secs(2));
        READY.store(true, Release);
    });

    // READYがloadした後は観測できる
    while !READY.load(Acquire) {
        thread::sleep(Duration::from_millis(100));
        println!("waiting..");
    }
    println!("{}", DATA.load(Relaxed));
}
