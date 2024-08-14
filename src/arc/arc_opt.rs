use std::cell::UnsafeCell;
use std::mem::ManuallyDrop;
use std::ops::Deref;
use std::ptr::NonNull;
use std::sync::atomic::{
    fence, AtomicUsize,
    Ordering::{Acquire, Relaxed, Release},
};

pub struct Arc<T> {
    ptr: NonNull<ArcData<T>>,
}

unsafe impl<T> Send for Arc<T> where T: Sync + Send {}
unsafe impl<T> Sync for Arc<T> where T: Sync + Send {}

impl<T> Arc<T> {
    pub fn new(data: T) -> Self {
        Self {
            ptr: NonNull::from(Box::leak(Box::new(ArcData {
                alloc_ref_count: AtomicUsize::new(1),
                data_ref_count: AtomicUsize::new(1),
                data: UnsafeCell::new(ManuallyDrop::new(data)),
            }))),
        }
    }

    fn data(&self) -> &ArcData<T> {
        unsafe { self.ptr.as_ref() }
    }
}

impl<T> Deref for Arc<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        // 安全性: このデータに対するArcが存在するので、
        // データは存在し、共有されている可能性もある
        unsafe { &*self.data().data.get() }
    }
}

impl<T> Clone for Arc<T> {
    fn clone(&self) -> Self {
        if self.data().data_ref_count.fetch_add(1, Relaxed) > usize::MAX / 2 {
            std::process::abort();
        }

        Self { ptr: self.ptr }
    }
}

pub struct Weak<T> {
    ptr: NonNull<ArcData<T>>,
}

impl<T> Weak<T> {
    fn data(&self) -> &ArcData<T> {
        unsafe { self.ptr.as_ref() }
    }
}

impl<T> Clone for Weak<T> {
    fn clone(&self) -> Self {
        if self.data().alloc_ref_count.fetch_add(1, Relaxed) > usize::MAX / 2 {
            std::process::abort();
        }

        Weak { ptr: self.ptr }
    }
}

impl<T> Drop for Weak<T> {
    fn drop(&mut self) {
        if self.data().alloc_ref_count.fetch_sub(1, Release) == 1 {
            fence(Acquire);
            unsafe {
                drop(Box::from_raw(self.ptr.as_ptr()));
            }
        }
    }
}

struct ArcData<T> {
    /// `Arc`の数
    data_ref_count: AtomicUsize,
    /// `Weak`の数`Arc`が一個でもあったらさらに一個足す
    alloc_ref_count: AtomicUsize,
    /// データ: Wekaしかなくなったらdropされる
    data: UnsafeCell<ManuallyDrop<T>>,
}
