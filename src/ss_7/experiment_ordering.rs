use std::sync::atomic::{
    compiler_fence, AtomicBool, AtomicUsize,
    Ordering::{Acquire, Relaxed, Release},
};
use std::thread;

// X86_64だと期待したように(本来は間違っているが)動いてしまう。
// aarch64だと間違った結果となる
fn main() {
    let locked = AtomicBool::new(false);
    let counter = AtomicUsize::new(0);

    // 4スレッドを起動し、1000万回繰り返す
    thread::scope(|s| {
        for _ in 0..4 {
            s.spawn(|| {
                for _ in 0..1_000_000 {
                    // ロックを取得
                    while locked.swap(true, Relaxed) {}
                    compiler_fence(Acquire);

                    // ロックを保持したまま、非アトミックにカウンタをインクリメント
                    let old = counter.load(Relaxed);
                    let new = old + 1;
                    counter.store(new, Relaxed);

                    // ロックを開放する
                    // メモリーオーダリングが間違っている
                    compiler_fence(Release);
                    locked.store(false, Relaxed);
                }
            });
        }
    });

    println!("{}", counter.into_inner());
}
