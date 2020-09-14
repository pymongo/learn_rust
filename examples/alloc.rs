#![feature(allocator_api)]
use std::alloc::{System, Layout, GlobalAlloc};

struct MyBox<T> {
    val: *const T
}

impl<T> MyBox<T> {
    fn new(val: T) -> Self {
        unsafe {
            let p = System.alloc(Layout::array::<T>(1).unwrap());
            let p = p as *mut T;
            std::ptr::write(p, val);
            MyBox { val: p }
        }
    }
}

impl<T> Drop for MyBox<T> {
    fn drop(&mut self) {
        unsafe {
            // raw pointer的covariant(协变)、逆变(contravariant)、不变(invariant)
            let p = self.val as *mut _;
            System.dealloc(p, Layout::array::<T>(std::mem::align_of::<T>()).unwrap());
        }
    }
}

fn main() {
    let _a = MyBox::new(0i32);
}