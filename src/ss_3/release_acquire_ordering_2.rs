use std::sync::atomic::{
    AtomicBool,
    Ordering::{Acquire, Release},
};
use std::thread;
use std::time::Duration;

static mut DATA: u64 = 0;
static READY: AtomicBool = AtomicBool::new(false);

// Release: ストア操作に適用
// Acquire: ロード操作に適用
fn main() {
    thread::spawn(|| {
        unsafe { DATA = 123 };
        thread::sleep(Duration::from_secs(2));
        READY.store(true, Release);
    });

    // READYがloadした後は観測できる
    while !READY.load(Acquire) {
        thread::sleep(Duration::from_millis(100));
        println!("waiting..");
    }
    println!("{}", unsafe { DATA });
}
