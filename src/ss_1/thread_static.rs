use std::thread;

fn main() {
    static nums: [i32; 3] = [1, 2, 3];

    let t_1 = thread::spawn(|| dbg!(&nums));
    let t_2 = thread::spawn(|| dbg!(&nums));

    t_1.join().unwrap();
    t_2.join().unwrap();
}
