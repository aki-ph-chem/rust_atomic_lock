use std::sync::atomic::{
    fence,
    Ordering::{AcqRel, Acquire, Release, SeqCst},
};

#[no_mangle]
pub fn fence_acquire() {
    fence(Acquire);
}

#[no_mangle]
pub fn fence_release() {
    fence(Release);
}

#[no_mangle]
pub fn fence_acqrel() {
    fence(AcqRel);
}

#[no_mangle]
pub fn fence_seqcst() {
    fence(SeqCst);
}
