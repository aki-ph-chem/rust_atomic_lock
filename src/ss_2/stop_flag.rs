use std::sync::atomic::{AtomicBool, Ordering::Relaxed};
use std::thread;

fn some_work() {}

fn main() {
    static STOP: AtomicBool = AtomicBool::new(false);

    // スレッドを起動
    let background_thread = thread::spawn(|| {
        while !STOP.load(Relaxed) {
            some_work();
        }
    });

    // メインスレッドで入力を受ける
    for line in std::io::stdin().lines() {
        match line.unwrap().as_str() {
            "help" => println!("commands: help, stop"),
            "stop" => break,
            cmd => println!("unknown command: {cmd:?}"),
        }
    }

    // バックグラウンドスレッドに停止を通知
    STOP.store(true, Relaxed);

    //  スレッドの終了を待つ
    background_thread.join().unwrap();
}
