use rust_atomic_lock::channel_type;
use std::thread;

fn main() {
    thread::scope(|s| {
        let (sender, receiver) = channel_type::channel::<&str>();
        let t = thread::current();

        // 送信スレッド
        s.spawn(move || {
            sender.send("Hello World");
            // 一度メッセージを送ったらメインスレッドをunparkする
            t.unpark();
        });

        // 受信スレッド(メインスレッド)はpark()で受信するまで待機
        while !receiver.is_ready() {
            thread::park();
        }
        // 受信
        assert_eq!(receiver.receive(), "Hello World");
    });
}
