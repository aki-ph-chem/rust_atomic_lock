use std::collections::VecDeque;
use std::sync::Mutex;
use std::thread;
use std::time::Duration;

fn main() {
    let queue = Mutex::new(VecDeque::<i32>::new());

    thread::scope(|s| {
        // 消費スレッド
        // 無限ループの中で、キューから値を取り出して、表示
        // 空ならスレッドをパークする(スリープする)
        let t = s.spawn(|| loop {
            let item = queue.lock().unwrap().pop_front();
            if let Some(item) = item {
                dbg!(item);
            }
            // elseの部分(パークする処理)はなくても正しいプログラムになる
            else {
                println!("parked");
                thread::park();
            }
        });

        // 生成スレッド
        // 毎秒キューに値を追加し、消費スレッドtを起動する
        for i in 0.. {
            println!("{i} is pushed");
            queue.lock().unwrap().push_back(i);
            t.thread().unpark();
            thread::sleep(Duration::from_secs(1));
        }
    });
}
