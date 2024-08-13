use rust_atomic_lock::channel_ref::Channel;
use std::thread;

fn main() {
    let mut channel = Channel::<&str>::new();
    thread::scope(|s| {
        let (sender, receiver) = channel.split();
        s.spawn(move || {
            sender.send("Hello World");
        });

        assert_eq!(receiver.receive(), "Hello World");
    });
}
