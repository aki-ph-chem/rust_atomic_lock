use rust_atomic_lock::channel::channel_save_mem::Channel;
use std::thread;

fn main() {
    let channel = Channel::<&str>::new();
    // t: 受信スレッド(メインスレッド)
    let t = thread::current();

    thread::scope(|s| {
        s.spawn(|| {
            channel.send("Hello World");
            // 一度メッセージを送ったらメインスレッドをunparkする
            t.unpark();
        });

        // 受信スレッド(メインスレッド)はpark()で受信するまで待機
        while !channel.is_ready() {
            thread::park();
        }
        assert_eq!(channel.receive(), "Hello World");
    });
}
