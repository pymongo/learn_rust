use std::sync::{Arc, Barrier};

const N_THREADS: usize = 5;

// 通过Barrier让乱序执行的5个线程强行"有序"，5个线程全部执行完前半部分操作后，再开始后半部分操作
// 例如学校组织同学们去春游或公司团建，必须等班上所有同学都上了大巴车之后，大巴才能发车去下一个景点，不会让任何同学(线程)掉队
fn main() {
    let mut handles = Vec::with_capacity(N_THREADS);
    // Creates a new barrier that can block a given number of threads
    let barrier = Arc::new(Barrier::new(N_THREADS));
    for _ in 0..N_THREADS {
        let barrier_c = barrier.clone();
        handles.push(std::thread::spawn(move || {
            println!("Before wait");
            barrier_c.wait();
            println!("After wait");
        }));
    }
    for handle in handles {
        handle.join().unwrap();
    }
}
