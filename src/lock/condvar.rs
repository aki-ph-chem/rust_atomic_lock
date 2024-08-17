use crate::mutex_opt::{Mutex, MutexGuard};
use atomic_wait;
use std::sync::atomic::{AtomicU32, Ordering::Relaxed};
use std::thread;
use std::time::Duration;

pub struct Condvar {
    counter: AtomicU32,
}

impl Condvar {
    pub const fn new() -> Self {
        Self {
            counter: AtomicU32::new(0),
        }
    }

    // 通知メソッド

    pub fn notify_one(&self) {
        self.counter.fetch_add(1, Relaxed);
        atomic_wait::wake_one(&self.counter);
    }

    pub fn notify_all(&self) {
        self.counter.fetch_add(1, Relaxed);
        atomic_wait::wake_all(&self.counter);
    }

    pub fn wait<'a, T>(&self, guard: MutexGuard<'a, T>) -> MutexGuard<'a, T> {
        let counter_value = self.counter.load(Relaxed);

        // ガードをdropしてアンロック
        // ただ、後でlockするためにmutexを保持しておく
        let mutex = guard.mutex;
        drop(guard);

        // カウンタ値がアンロックする前から更新されていない場合のみ待機
        atomic_wait::wait(&self.counter, counter_value);

        mutex.lock()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_condvar() {
        let mutex = Mutex::new(0);
        let condvar = Condvar::new();

        let mut wakeups = 0;
        thread::scope(|s| {
            s.spawn(|| {
                thread::sleep(Duration::from_secs(1));
                *mutex.lock() = 123;
                condvar.notify_one();
            });

            let mut m = mutex.lock();
            while *m < 100 {
                m = condvar.wait(m);
                wakeups += 1;
            }

            assert_eq!(*m, 123);
        });

        // メインスレッドが実際にwaitしたことをチェック
        // ただし、何度か誤って起こることは許容する
        assert!(wakeups < 10);
    }
}
