use rand::{self, Rng};
use std::sync::atomic::AtomicPtr;
use std::sync::atomic::Ordering::{Acquire, Release};
use std::thread;

// 遅延初期化をノンブロッキングで実装
fn get_data_i32() -> &'static i32 {
    // AtomicPtr<T>は*mut Tのアトミック版
    static PTR: AtomicPtr<i32> = AtomicPtr::new(std::ptr::null_mut());
    let mut p = PTR.load(Acquire);

    if p.is_null() {
        p = Box::into_raw(Box::new(gen_i32()));
        if let Err(err) = PTR.compare_exchange(std::ptr::null_mut(), p, Release, Acquire) {
            // pは直前のBox::into_raw()で作ったものなので、
            // 他のスレッドと共有されることはない
            drop(unsafe { Box::from_raw(p) });
            p = err;
        }
    }

    unsafe { &*p }
}

fn gen_i32() -> i32 {
    let mut rand = rand::thread_rng();
    rand.gen()
}

fn main() {
    thread::scope(|s| {
        for _ in 0..5 {
            s.spawn(|| {
                let n = get_data_i32();
                println!("n: {}", n)
            });
        }
    });
}
