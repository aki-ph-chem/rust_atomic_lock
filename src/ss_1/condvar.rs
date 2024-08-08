use std::collections::VecDeque;
use std::sync::{Condvar, Mutex};
use std::thread;
use std::time::Duration;

fn main() {
    let queue = Mutex::new(VecDeque::<i32>::new());
    let not_empty = Condvar::new();

    thread::scope(|s| {
        // 消費スレッド
        s.spawn(|| loop {
            let mut q = queue.lock().unwrap();
            let item = loop {
                if let Some(item) = q.pop_front() {
                    break item;
                } else {
                    // スリープする
                    q = not_empty.wait(q).unwrap();
                }
            };

            drop(q);
            dbg!(item);
        });

        // 生成スレッド
        for i in 0.. {
            println!("{i} is pushed");
            queue.lock().unwrap().push_back(i);
            // スレッドを起こす
            not_empty.notify_one();
            thread::sleep(Duration::from_secs(1));
        }
    });
}
