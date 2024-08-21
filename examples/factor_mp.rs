use std::thread;
use std::time;

/// シングルスレッドで約数の個数を数え上げる
fn factors_sp(num: i64) -> i64 {
    let mut result = 0;
    for i in 1..=num {
        if num % i == 0 {
            result += 1;
        }
    }

    result
}

struct FactTask {
    num: i64,
    from: i64,
    to: i64,
}

impl FactTask {
    pub fn new(num: i64, from: i64, to: i64) -> Self {
        Self { num, from, to }
    }

    pub fn factor(&self) -> i64 {
        let mut result = 0;
        for i in self.from..self.to {
            if self.num % i == 0 {
                result += 1;
            }
        }

        result
    }
}

/// マルチスレッドで約数の個数を数え上げる
fn factors_mp(num: i64, num_thread: i64) -> i64 {
    let (mut start, step, residual) = (1, num / num_thread, num % num_thread);

    // スレッドを建てる
    let mut threads = vec![];
    for _i in 0..num_thread {
        let task = FactTask::new(num, start, start + step);
        start += step;
        threads.push(thread::spawn(move || task.factor()));
    }

    // スレッドの待機
    let mut result = 0;
    for t in threads {
        result += t.join().unwrap();
    }

    // 端数の処理(メインスレッドで処理)
    if residual != 0 {
        for i in start..=num {
            if num % i == 0 {
                result += 1;
            }
        }
    }

    result
}

fn main() {
    let num = 2_000_000_000;

    // シングルスレッド
    let start = time::Instant::now();
    let factor = factors_sp(num);
    println!(
        "thread: 1, number of factor of {num}: {factor}, time: {:?}",
        start.elapsed()
    );

    // 4スレッド
    let start = time::Instant::now();
    let factor = factors_mp(num, 4);
    println!(
        "thread: 4, number of factor of {num}: {factor}, time: {:?}",
        start.elapsed()
    );

    // 6スレッド
    let start = time::Instant::now();
    let factor = factors_mp(num, 6);
    println!(
        "thread: 6, number of factor of {num}: {factor}, time: {:?}",
        start.elapsed()
    );

    // 8スレッド
    let start = time::Instant::now();
    let factor = factors_mp(num, 8);
    println!(
        "thread: 8, number of factor of {num}: {factor}, time: {:?}",
        start.elapsed()
    );

    // 10スレッド
    let start = time::Instant::now();
    let factor = factors_mp(num, 10);
    println!(
        "thread: 10, number of factor of {num}: {factor}, time: {:?}",
        start.elapsed()
    );

    // 12スレッド
    let start = time::Instant::now();
    let factor = factors_mp(num, 12);
    println!(
        "thread: 12, number of factor of {num}: {factor}, time: {:?}",
        start.elapsed()
    );

    // 16スレッド
    let start = time::Instant::now();
    let factor = factors_mp(num, 16);
    println!(
        "thread: 16, number of factor of {num}: {factor}, time: {:?}",
        start.elapsed()
    );

    // 32スレッド
    let start = time::Instant::now();
    let factor = factors_mp(num, 32);
    println!(
        "thread: 32, number of factor of {num}: {factor}, time: {:?}",
        start.elapsed()
    );
}
