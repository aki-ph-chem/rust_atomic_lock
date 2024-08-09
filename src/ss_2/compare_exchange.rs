use std::sync::atomic::{AtomicU32, Ordering::Relaxed};

// compare_exchange()
// アトミックな値が与えられた値と等しければ交換する

// compare_exchange()を使う
fn increment(a: &AtomicU32) {
    // aをロード
    let mut current = a.load(Relaxed);
    loop {
        // aの値をインクリメントした値を計算
        let new = current + 1;
        match a.compare_exchange(current, new, Relaxed, Relaxed) {
            // 値が更新されていなければ終了
            Ok(_) => return,
            // 更新されていた場合はcurrentを更新(別スレッドによって更新された)
            Err(v) => current = v,
        }
    }
}

fn main() {
    let n = AtomicU32::new(99);
    println!("n: {:?}", n);
    increment(&n);
    println!("n: {:?}", n);
}
