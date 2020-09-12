// park好像是Unix线程的一个API
fn main() {
    // 除了阻塞/重启/join等的同步方法，Rust还提供yield_now让线程主动出让当前时间片
    let parked_thread = std::thread::spawn(|| {
        println!("Parking thread");
        // park函数并不能永久阻塞线程，会有一个默认的park_timeout，也可以显式指定park_timeout
        std::thread::park();
        // 线程会从暂停的上下文处往下执行
        println!("thread unparked");
    });
    std::thread::sleep(std::time::Duration::from_millis(200));
    parked_thread.thread().unpark();
    parked_thread.join().unwrap();
}
