use std::sync::atomic::{AtomicUsize, Ordering::Relaxed};
use std::thread;
use std::time::Duration;

fn process_item(_i: usize) {
    // 100 ms待つ
    thread::sleep(Duration::from_millis(100));
}

fn main() {
    // atomicな数値型はCopyを実装していない
    let num_done = &AtomicUsize::new(0);

    thread::scope(|s| {
        // スレッドを4個建てて25個ずつアイテムを処理
        for t in 0..4 {
            s.spawn(move || {
                for i in 0..25 {
                    process_item(t * 25 + i);
                    num_done.fetch_add(1, Relaxed);
                }
            });
        }

        // メインスレッド
        loop {
            let n = num_done.load(Relaxed);
            if n == 100 {
                break;
            }

            println!("working.. {n}/100 done");
            thread::sleep(Duration::from_secs(1));
        }
    });

    println!("Done!");
}
