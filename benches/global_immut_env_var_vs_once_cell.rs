/*!
探索共享全局字符串常量的最佳实践，例如我要通过dotenv从配置文件中读取一个字符串，用作全局变量
test bench_env_var   ... bench:         255 ns/iter (+/- 16)
test bench_once_cell ... bench:           1 ns/iter (+/- 0)
*/
#![feature(test, once_cell)]
extern crate test;
use std::lazy::SyncOnceCell;

static MARKET_ID: SyncOnceCell<String> = SyncOnceCell::new();

#[bench]
fn bench_env_var(bencher: &mut test::Bencher) {
    std::env::set_var("MARKET_ID", "btcusdt");
    bencher.iter(|| {
        assert_eq!(std::env::var("MARKET_ID").unwrap(), "btcusdt");
    });
}

#[bench]
fn bench_once_cell(bencher: &mut test::Bencher) {
    MARKET_ID.set("btcusdt".to_string()).unwrap();
    bencher.iter(|| {
        assert_eq!(MARKET_ID.get().unwrap(), "btcusdt");
    });
}
