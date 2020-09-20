#![feature(test, once_cell)]
extern crate test;
// benchmark为了测试最佳性能，都是release级编译，暂无--debug的选项
// cargo +nightly bench --bench get_decimal_zero

use bigdecimal::{BigDecimal, Zero};
use std::lazy::SyncOnceCell;

static ZERO: SyncOnceCell<BigDecimal> = SyncOnceCell::new();

#[bench]
fn global_zero(bencher: &mut test::Bencher) {
    ZERO.set(BigDecimal::zero()).unwrap();
    // 由于目前版本的Rust不支持async bench，所以无法读取数据库，只能用eq来模拟读取&BigDecimal的过程
    bencher.iter(|| {
        let zero = ZERO.get().unwrap();
        assert!(BigDecimal::zero().eq(zero));
        assert!(BigDecimal::from(1).ne(zero));
    })
}

/*
test global_zero ... bench:          76 ns/iter (+/- 20)
test new_zero    ... bench:          81 ns/iter (+/- 142)
*/
#[bench]
fn new_zero(bencher: &mut test::Bencher) {
    bencher.iter(|| {
        assert!(BigDecimal::zero().eq(&BigDecimal::zero()));
        assert!(BigDecimal::from(1).ne(&BigDecimal::zero()))
    })
}
