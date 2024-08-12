use std::sync::atomic::{
    fence, AtomicBool,
    Ordering::{Acquire, Relaxed},
};
use std::thread;
use std::time::Duration;

static mut DATA: [u64; 10] = [0; 10];
const ATOMIC_FALSE: AtomicBool = AtomicBool::new(false);
static READY: [AtomicBool; 10] = [ATOMIC_FALSE; 10];

fn some_calculation(_i: usize) -> u64 {
    1
}

fn main() {
    // Release/Acquire フェンス
    for i in 0..10 {
        thread::spawn(move || {
            let data = some_calculation(i);
            unsafe { DATA[i] = data };
        });
    }
    thread::sleep(Duration::from_millis(500));
    let ready: [bool; 10] = std::array::from_fn(|i| READY[i].load(Relaxed));
    if ready.contains(&true) {
        fence(Acquire);
        for i in 0..10 {
            if ready[i] {
                println!("data[{i}] = {}", unsafe { DATA[i] });
            }
        }
    }
}
