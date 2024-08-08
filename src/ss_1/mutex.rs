use std::sync::Mutex;
use std::thread;
use std::time::Duration;

// Mutex<T>: スレッド間で共有される

fn main() {
    let n = Mutex::new(0);
    // スレッドを10個建ててそれぞれで100回インクリメントする
    thread::scope(|s| {
        for _ in 0..10 {
            s.spawn(|| {
                // lock()でロックを取る
                let mut guard = n.lock().unwrap();
                for _ in 0..100 {
                    *guard += 1;
                }
                // スリープするよりも先にガードをドロップする
                drop(guard);
                thread::sleep(Duration::from_secs(1));
            });
        }
    });

    // into_inner()で中身を取ら出す
    assert_eq!(n.into_inner().unwrap(), 1000);
}
