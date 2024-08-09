use std::sync::atomic::{AtomicU32, Ordering::Relaxed};

// 呼ばれるたびにユニークなIDを返す
// 問題あり
fn allocate_new_id() -> u32 {
    static NEXT_ID: AtomicU32 = AtomicU32::new(0);
    let id = NEXT_ID.fetch_add(1, Relaxed);
    // 1001回目でパニックする
    // しかし、別スレッドで1002回目のインクリメントが実行される可能性がある
    assert!(id < 1000, "too many IDs");

    id
}

// 解決法1
// 1001回目でstd::process::abort()でプログラム全体を終了する
// Arc::cloneはこの方法を採用している
fn allocate_new_id_abort() -> u32 {
    static NEXT_ID: AtomicU32 = AtomicU32::new(0);
    let id = NEXT_ID.fetch_add(1, Relaxed);
    if id >= 1000 {
        eprintln!("too many IDs");
        std::process::abort();
    }

    id
}

// 解決法2
// パニックする前にfetch_sub()でカウンタをデクリメントすることでオーバーフローを回避する
// thread::scopeではこの方法を採用している
fn allocate_new_id_decrement() -> u32 {
    static NEXT_ID: AtomicU32 = AtomicU32::new(0);
    let id = NEXT_ID.fetch_add(1, Relaxed);
    if id >= 1000 {
        NEXT_ID.fetch_sub(1, Relaxed);
        panic!("too many IDs");
    }

    id
}

// 解決法3
// オーバーフローする可能性がある場合は加算を行わない
// 比較交換操作が要るのでここでは述べない

fn main() {
    for i in 0..10 {
        let id = allocate_new_id();
        println!("{i} th: id: {id}");
    }

    for i in 0..10 {
        let id = allocate_new_id_abort();
        println!("{i} th: id: {id}");
    }

    for i in 0..10 {
        let id = allocate_new_id_decrement();
        println!("{i} th: id: {id}");
    }
}
