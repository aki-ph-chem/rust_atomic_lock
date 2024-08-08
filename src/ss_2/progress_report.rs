use std::sync::atomic::{AtomicUsize, Ordering::Relaxed};
use std::thread;
use std::time::Duration;

fn process_item(_i: usize) {
    // 100 ms待つ
    thread::sleep(Duration::from_millis(100));
}

fn main() {
    let num_done = AtomicUsize::new(0);

    // バックグラウンドスレッドを建てて100個のアイテムを全て処理する
    thread::scope(|s| {
        s.spawn(|| {
            for i in 0..100 {
                process_item(i); // 本来時間のかかる処理
                num_done.store(i + 1, Relaxed);
            }
        });

        // メインスレッドでは毎秒1回状態をチェックして更新する
        loop {
            let n = num_done.load(Relaxed);
            if n == 100 {
                break;
            }

            println!("working: {n}/100 done");
            thread::sleep(Duration::from_secs(1));
        }
    });

    println!("Done!");
}
