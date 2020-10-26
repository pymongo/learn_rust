/*!
test bench_async_read_files ... bench:      95,751 ns/iter (+/- 4,091)
test bench_sync_read_files  ... bench:     214,285 ns/iter (+/- 32,424)
*/
#![feature(test)]
extern crate test;
use itertools::Itertools;
use std::io::Read;

const FILE_PATH: &str = "Cargo.toml";
const READ_TIMES: usize = 10;

#[bench]
fn bench_sync_read_files(bencher: &mut test::Bencher) {
    bencher.iter(|| {
        let thread_handlers = (0..READ_TIMES)
            .map(|_| {
                std::thread::spawn(|| {
                    let mut file = std::fs::File::open(FILE_PATH).unwrap();
                    file.read_to_string(&mut String::new()).unwrap();
                })
            })
            .collect_vec();
        for handler in thread_handlers {
            handler.join().unwrap();
        }
    });
}

#[bench]
fn bench_async_read_files(bencher: &mut test::Bencher) {
    bencher.iter(|| {
        futures::executor::block_on(async {
            futures::future::join_all(
                (0..READ_TIMES).map(|_| async_std::fs::read_to_string(FILE_PATH)),
            )
            .await;
        });
    });
}
