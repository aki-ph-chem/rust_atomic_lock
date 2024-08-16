use libc;
use std::sync::atomic::{AtomicU32, Ordering::Relaxed};
use std::thread;
use std::time;

#[cfg(not(target_os = "linux"))]
compile_error!("Linux only. Sorry!");

/// FUTEX_WAITをラップした関数
pub fn wait(a: &AtomicU32, expected: u32) {
    unsafe {
        libc::syscall(
            libc::SYS_futex,                    // futexシステムコール
            a as *const AtomicU32,              // 操作対象のアトミック変数
            expected,                           // 想定される値
            std::ptr::null::<libc::timespec>(), // タイムアウトはしない
        );
    }
}

/// FUTEX_WAKEをラップした関数
pub fn wake_one(a: &AtomicU32) {
    // このシステムコールのシグニチャはfutex (2)のmanページを参照
    unsafe {
        libc::syscall(
            libc::SYS_futex,       // futexシステムコール
            a as *const AtomicU32, // 操作対象のアトミック変数
            libc::FUTEX_WAKE,      // futex操作
            1,                     // 起こすスレッドの数
        );
    }
}

fn main() {
    let a = AtomicU32::new(0);

    thread::scope(|s| {
        // 別スレッド
        s.spawn(|| {
            thread::sleep(time::Duration::from_secs(3));
            a.store(1, Relaxed);
            wake_one(&a);
        });

        // メインスレッド
        // aが0である限り待機
        println!("Waiting...");
        while a.load(Relaxed) == 0 {
            wait(&a, 0);
        }
        println!("Done!");
    });
}
