use rust_atomic_lock::channel_ref::Channel;
use std::thread;

fn main() {
    let mut channel = Channel::<&str>::new();
    thread::scope(|s| {
        let (sender, receiver) = channel.split();
        let t = thread::current();

        s.spawn(move || {
            sender.send("Hello World");
            t.unpark();
        });
        while !receiver.is_ready() {
            thread::park();
        }

        assert_eq!(receiver.receive(), "Hello World");
    });
}
