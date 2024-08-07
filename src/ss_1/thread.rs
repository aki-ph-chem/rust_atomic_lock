use std::thread;

fn main() {
    // ２つスレッドを建てる
    let t_1 = thread::spawn(f);
    let t_2 = thread::spawn(f);

    // スレッドと所有権
    let nums = vec![1, 2, 3];
    let t_num = thread::spawn(move || {
        println!("from thread id: {:?}", thread::current().id());
        for n in nums {
            print!("{n} ");
        }
        println!("");
    });

    // スレッドから値を返す
    let nums = (0..=100).collect::<Vec<i32>>();
    let t_avg = thread::spawn(move || nums.iter().fold(0, |sum, x| sum + x) / nums.len() as i32);

    // 終了を待機
    t_1.join().unwrap();
    t_2.join().unwrap();
    t_num.join().unwrap();
    let avg = t_avg.join().unwrap();
    println!("avg = {avg}");
}

fn f() {
    println!("Hello from another thread");
    let id = thread::current().id();

    println!("This is my thread id: {id:?}");
}
