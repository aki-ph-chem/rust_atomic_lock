use std::sync::atomic::{AtomicU64, AtomicUsize, Ordering::Relaxed};
use std::thread;
use std::time::{Duration, Instant};

fn process_item(_i: usize) {
    // 100 ms待つ
    thread::sleep(Duration::from_millis(100));
}

fn main() {
    let num_done = &AtomicUsize::new(0);
    let total_time = &AtomicU64::new(0);
    let max_time = &AtomicU64::new(0);

    thread::scope(|s| {
        // スレッドを4個建てて25個ずつアイテムを処理
        for t in 0..4 {
            s.spawn(move || {
                for i in 0..25 {
                    let start = Instant::now();
                    process_item(t * 25 + i);
                    // process_itemでかかった時間
                    let time_taken = start.elapsed().as_micros() as u64;
                    // 回数をインクリメント
                    num_done.fetch_add(1, Relaxed);
                    // トータルのタイムを計算
                    total_time.fetch_add(time_taken, Relaxed);
                    // time_takenの最大値を計算
                    max_time.fetch_max(time_taken, Relaxed);
                }
            });
        }

        // メインスレッドは更新された状態を毎秒表示する
        loop {
            let total_time = Duration::from_micros(total_time.load(Relaxed));
            let max_time = Duration::from_micros(max_time.load(Relaxed));
            let n = num_done.load(Relaxed);
            if n == 100 {
                break;
            }
            if n == 0 {
                println!("Working.. nothing done yet");
            } else {
                // 進捗と平均実行時間、最大実行時間を表示
                println!(
                    "working.. {n}/100 done, {:?} average, {:?} peak",
                    total_time / n as u32,
                    max_time
                );
            }
            thread::sleep(Duration::from_secs(1));
        }
    });

    println!("Done!");
}
