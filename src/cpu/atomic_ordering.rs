use std::sync::atomic::{
    AtomicI32,
    Ordering::{AcqRel, Acquire, Release, SeqCst},
};

// Release,Acquire,AcqRel

#[no_mangle]
pub fn store_release(x: &AtomicI32) {
    x.store(0, Release);
}

#[no_mangle]
pub fn load_acquire(x: &AtomicI32) {
    x.load(Acquire);
}

#[no_mangle]
pub fn fetch_add_acqrel(x: &AtomicI32) {
    x.fetch_add(10, AcqRel);
}

// SeqCst

#[no_mangle]
pub fn store_seqcst(x: &AtomicI32) {
    x.store(0, SeqCst);
}

#[no_mangle]
pub fn load_seqcst(x: &AtomicI32) {
    x.load(SeqCst);
}

#[no_mangle]
pub fn fetch_add_seqcst(x: &AtomicI32) {
    x.fetch_add(10, SeqCst);
}
