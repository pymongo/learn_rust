use std::sync::{
    atomic::{AtomicU32, Ordering},
    Arc,
};

fn main() {
    let spinlock = Arc::new(AtomicU32::new(1));
    let spinlock_clone = spinlock.clone();
    let thread = std::thread::spawn(move || {
        spinlock_clone.store(0, Ordering::SeqCst);
    });
    // 自旋锁: 不断轮询直到能获取锁资源
    println!("before while");
    while spinlock.load(Ordering::SeqCst) != 0 {
        println!("acquire spinlock failed");
    }
    println!("after while");
    thread.join().unwrap();
    println!("after join");
}
