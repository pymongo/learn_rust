/*! tokio的M(os thread)/N(green thread)的性能碾压futures
test bench_async_async_std                      ... bench:     104,534 ns/iter (+/- 53,638)
test bench_async_futures_single_thread_executor ... bench:     112,197 ns/iter (+/- 53,057)
test bench_async_tokio_multi_threads            ... bench:         620 ns/iter (+/- 7,581)
test bench_sync                                 ... bench:   1,069,225 ns/iter (+/- 4,256,327)
*/
#![feature(test)]
extern crate test;
use std::io::Read;

const FILE_PATH: &str = "Cargo.toml";
const READ_TIMES: usize = 10;

#[bench]
fn bench_sync(bencher: &mut test::Bencher) {
    bencher.iter(|| {
        let thread_handlers = (0..READ_TIMES)
            .map(|_| {
                std::thread::spawn(|| {
                    let mut file = std::fs::File::open(FILE_PATH).unwrap();
                    file.read_to_string(&mut String::new()).unwrap();
                })
            })
            .collect::<Vec<_>>();
        for handler in thread_handlers {
            handler.join().unwrap();
        }
    });
}

async fn read_files() {
    futures::future::join_all((0..READ_TIMES).map(|_| async_std::fs::read_to_string(FILE_PATH)))
        .await;
}

#[bench]
fn bench_async_futures_single_thread_executor(bencher: &mut test::Bencher) {
    bencher.iter(|| futures::executor::block_on(read_files()));
}

#[bench]
fn bench_async_tokio_multi_threads(bencher: &mut test::Bencher) {
    let rt = tokio::runtime::Runtime::new().unwrap();
    bencher.iter(|| rt.spawn(read_files()));
}

#[bench]
fn bench_async_async_std(bencher: &mut test::Bencher) {
    bencher.iter(|| async_std::task::block_on(read_files()));
}

#[bench]
#[ignore]
fn bench_async_futures_multi_threads_executor(bencher: &mut test::Bencher) {
    // FIXME would stuck
    let pool = futures::executor::ThreadPool::new().unwrap();
    bencher.iter(|| pool.spawn_ok(read_files()));
}

/**
futures::executor::ThreadPool: 64.484µs
tokio::runtime::Runtime: 542.913µs
*/
// #[test]
#[cfg(FALSE)]
fn test_async_multi_threads_executor() {
    let pool = futures::executor::ThreadPool::new().unwrap();
    let now = std::time::Instant::now();
    pool.spawn_ok(async {
        futures::future::join_all(
            (0..READ_TIMES).map(|_| async_std::fs::read_to_string(FILE_PATH)),
        )
        .await;
    });
    println!("futures::executor::ThreadPool: {:?}", now.elapsed());
    drop(pool);

    let rt = tokio::runtime::Runtime::new().unwrap();
    rt.spawn(async {
        futures::future::join_all(
            (0..READ_TIMES).map(|_| async_std::fs::read_to_string(FILE_PATH)),
        )
        .await;
    });
    println!("tokio::runtime::Runtime: {:?}", now.elapsed());
}
