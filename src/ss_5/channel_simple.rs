use rust_atomic_lock::channel::channel_simple::Channel;
use std::thread;

fn main() {
    let chan_1 = Channel::<String>::new();
    thread::scope(|s| {
        s.spawn(|| {
            // 別スレッドから"Hello!"をsend
            chan_1.send("Hello!".to_string());
        });
    });

    // 送信されたものをメインスレッドでreceive
    let message = chan_1.receive();
    println!("message: {message}");
}
