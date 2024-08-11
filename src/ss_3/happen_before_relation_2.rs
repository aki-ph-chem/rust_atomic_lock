use std::sync::atomic::{AtomicI32, Ordering::Relaxed};
use std::thread;

static X: AtomicI32 = AtomicI32::new(0);

fn f() {
    let x = X.load(Relaxed);
    assert!(x == 1 || x == 2); // 失敗しない
}

fn main() {
    // スレッドの起動とジョインの先行関係
    // からXからの読み込みは最初のXへのストアの後で実行される
    X.store(1, Relaxed);
    let t = thread::spawn(f);
    X.store(2, Relaxed);
    t.join().unwrap();

    X.store(3, Relaxed);
}
