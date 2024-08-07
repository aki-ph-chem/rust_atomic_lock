use std::thread;

fn main() {
    let nums = vec![1, 3, 5, 9];
    // スコープ付きスレッド
    thread::scope(|s| {
        s.spawn(|| {
            println!("from thread id: {:?}", thread::current().id());
            println!("length: {}", nums.len())
        });
        s.spawn(|| {
            println!("from thread id: {:?}", thread::current().id());
            let sum = nums.iter().fold(0, |sum, n| sum + n);
            println!("sum = {sum}");
        });
    });
}
