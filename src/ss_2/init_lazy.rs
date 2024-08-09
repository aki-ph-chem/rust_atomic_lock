use rand::{self, Rng};
use std::sync::atomic::{AtomicU64, Ordering::Relaxed};

// 遅延初期化
// 値Xは0であることは許されないとする
// この方法では別の一つ目のスレッドでget_x()計算中に2つ目のスレッドがget_x()を計算する(競合)可能性がある
fn get_x() -> u64 {
    // 0で初期化
    static X: AtomicU64 = AtomicU64::new(0);
    let mut x = X.load(Relaxed);

    // 初期値のままならランダムに値で初期化する
    if x == 0 {
        x = calculate_x();
        X.store(x, Relaxed);
    }

    x
}

// 初期化する用の関数
// 1 ~ 10の乱数を生成する
fn calculate_x() -> u64 {
    let mut rand = rand::thread_rng();
    rand.gen_range(1..=10)
}

fn main() {
    for _ in 0..5 {
        let n = get_x();
        println!("n: {n}");
    }
}
