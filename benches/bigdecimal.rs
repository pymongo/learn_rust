/*!
BigDecimal占据内存大小和性能都远不如rust_decimal，已弃用
本测试就是为了比较BigDecimal在运算过程中rhs和lhs都要使用指针性能才更好的特点
*/
#![feature(test, once_cell)]
extern crate bigdecimal;
extern crate test;

use bigdecimal::{BigDecimal, FromPrimitive};
use std::borrow::Borrow;
use std::lazy::SyncLazy;
use std::str::FromStr;

//////////////////// from和from_u64的性能对比 ////////////////////
#[bench]
#[ignore]
fn from(bencher: &mut test::Bencher) {
    bencher.iter(|| {
        let _volume: BigDecimal = BigDecimal::from(1);
    });
}

#[bench]
#[ignore]
fn from_u32(bencher: &mut test::Bencher) {
    bencher.iter(|| {
        let _volume: BigDecimal = BigDecimal::from_u32(1).unwrap();
    });
}

#[bench]
#[ignore]
fn from_u64(bencher: &mut test::Bencher) {
    bencher.iter(|| {
        let _volume: BigDecimal = BigDecimal::from_u64(1).unwrap();
    });
}

#[bench]
#[ignore]
fn from_i32(bencher: &mut test::Bencher) {
    bencher.iter(|| {
        let _volume: BigDecimal = BigDecimal::from_i32(1).unwrap();
    });
}

#[bench]
#[ignore]
/*
test from     ... bench: 64 ns/iter (+/- 11)
test from_i32 ... bench: 68 ns/iter (+/- 14)
test from_i64 ... bench: 70 ns/iter (+/- 15)
test from_u32 ... bench: 70 ns/iter (+/- 12)
test from_u64 ... bench: 73 ns/iter (+/- 3)
*/
fn from_i64(bencher: &mut test::Bencher) {
    bencher.iter(|| {
        let _volume: BigDecimal = BigDecimal::from_i64(1).unwrap();
    });
}
// ========================================

//////////////////// 单个运算符: move、ref、borrow性能对比 ////////////////////
// TODO 阅读bigdecimal源码`impl Mul<BigDecimal> for BigDecimal`
#[bench]
#[ignore]
fn one_operator_normal(bencher: &mut test::Bencher) {
    bencher.iter(|| {
        let price = BigDecimal::from_str("1.1").unwrap();
        let volume = BigDecimal::from(1);
        let _total = price * volume;
    });
}

#[bench]
#[ignore]
fn one_operator_lhs_ref(bencher: &mut test::Bencher) {
    bencher.iter(|| {
        let price = BigDecimal::from_str("1.1").unwrap();
        let volume = BigDecimal::from(1);
        let _total = &price * volume;
    });
}

#[bench]
#[ignore]
fn one_operator_rhs_ref(bencher: &mut test::Bencher) {
    bencher.iter(|| {
        let price = BigDecimal::from_str("1.1").unwrap();
        let volume = BigDecimal::from(1);
        let _total = price * &volume;
    });
}

#[bench]
#[ignore]
fn one_operator_both_ref(bencher: &mut test::Bencher) {
    bencher.iter(|| {
        let price = BigDecimal::from_str("1.1").unwrap();
        let volume = BigDecimal::from(1);
        let _total = &price * &volume;
    });
}

#[bench]
#[ignore]
fn one_operator_lhs_borrow(bencher: &mut test::Bencher) {
    bencher.iter(|| {
        let price = BigDecimal::from_str("1.1").unwrap();
        let volume = BigDecimal::from(1);
        let _total = price.borrow() * volume;
    });
}

#[bench]
#[ignore]
fn one_operator_rhs_borrow(bencher: &mut test::Bencher) {
    bencher.iter(|| {
        let price = BigDecimal::from_str("1.1").unwrap();
        let volume = BigDecimal::from(1);
        let _total = price * volume.borrow();
    });
}

#[bench]
#[ignore]
/*
test one_operator_both_borrow    ... bench: 503 ns/iter (+/- 80)
test one_operator_both_ref       ... bench: 506 ns/iter (+/- 87)
test one_operator_lhs_borrow     ... bench: 514 ns/iter (+/- 33)
test one_operator_lhs_ref        ... bench: 507 ns/iter (+/- 98)
test one_operator_normal         ... bench: 529 ns/iter (+/- 102)
test one_operator_rhs_borrow     ... bench: 515 ns/iter (+/- 81)
test one_operator_rhs_ref        ... bench: 530 ns/iter (+/- 88)

结合impl Mul源码以及测试结果得出结论：

- borrow()内部返回`&T`所以不考虑可读性，用&price比price.borrow()更好;deref()函数同理。
- 源码上看，左边是ref右边是普通的乘法运算代码最少，实际性能表现上也不错
- 源码上看，运算符左右两边都是ref，参数传递效率高，实际测试中性能最好
*/
fn one_operator_both_borrow(bencher: &mut test::Bencher) {
    bencher.iter(|| {
        let price = BigDecimal::from_str("1.1").unwrap();
        let volume = BigDecimal::from(1);
        let _total = price.borrow() * volume.borrow();
    });
}
// ========================================

//////////////////// 连续两次运算: clone、ref性能对比 ////////////////////
#[bench]
#[ignore]
fn two_mul_first_clone(bencher: &mut test::Bencher) {
    bencher.iter(|| {
        let price = BigDecimal::from_str("1.1").unwrap();
        let volume_a = BigDecimal::from(1);
        let volume_b = BigDecimal::from(1);
        // Error: let _total = volume_a * price +  volume_b * price;
        let _total = price.clone() * volume_a + price * volume_b;
    });
}

#[bench]
#[ignore]
fn two_mul_first_price_ref(bencher: &mut test::Bencher) {
    bencher.iter(|| {
        let price = BigDecimal::from_str("1.1").unwrap();
        let volume_a = BigDecimal::from(1);
        let volume_b = BigDecimal::from(1);
        let _total = &price * volume_a + price * volume_b;
    });
}

#[bench]
#[ignore]
fn two_mul_first_price_second_price_ref(bencher: &mut test::Bencher) {
    bencher.iter(|| {
        let price = BigDecimal::from_str("1.1").unwrap();
        let volume_a = BigDecimal::from(1);
        let volume_b = BigDecimal::from(1);
        let _total = &price * volume_a + &price * volume_b;
    });
}

#[bench]
#[ignore]
/*
test two_mul_both_ref                     ... bench:         722 ns/iter (+/- 83)
test two_mul_first_clone                  ... bench:         819 ns/iter (+/- 128)
test two_mul_first_price_ref              ... bench:         735 ns/iter (+/- 122)
test two_mul_first_price_second_price_ref ... bench:         720 ns/iter (+/- 121)
结论：运算符左边的使用指针，或者全用指针性能最好
*/
fn two_mul_both_ref(bencher: &mut test::Bencher) {
    bencher.iter(|| {
        let price = BigDecimal::from_str("1.1").unwrap();
        let volume_a = BigDecimal::from(1);
        let volume_b = BigDecimal::from(1);
        let _total = &price * &volume_a + &price * &volume_b;
    });
}
// ========================================
static MAX_RATIO: SyncLazy<BigDecimal> = SyncLazy::new(|| BigDecimal::from_str("1.1").unwrap());
static MIN_RATIO: SyncLazy<BigDecimal> = SyncLazy::new(|| BigDecimal::from_str("0.9").unwrap());

#[bench]
#[ignore]
fn last_price_lazy_static_ref(bencher: &mut test::Bencher) {
    bencher.iter(|| {
        let params_price = BigDecimal::from(1);
        let last_price = BigDecimal::from(1);
        if params_price > &last_price * &*MAX_RATIO || params_price < &last_price * &*MIN_RATIO {
            panic!("price > last_price*1.1 or price < last_price*0.9")
        }
    });
}

#[bench]
#[ignore]
/*
test last_price_lazy_static_borrow  ... bench:   860 ns/iter (+/- 127)
test last_price_lazy_static_ref     ... bench:   860 ns/iter (+/- 33)
*/
fn last_price_lazy_static_borrow(bencher: &mut test::Bencher) {
    bencher.iter(|| {
        let params_price = BigDecimal::from(1);
        let last_price = BigDecimal::from(1);
        if params_price > &last_price * (*MAX_RATIO).borrow()
            || params_price < &last_price * (*MIN_RATIO).borrow()
        {
            panic!("price > last_price*1.1 or price < last_price*0.9")
        }
    });
}
// ========================================

//////////////////// 运算比较符两边都用指针 ////////////////////
#[bench]
// #[ignore]
fn op_normal(bencher: &mut test::Bencher) {
    bencher.iter(|| {
        let params_price = BigDecimal::from(1);
        let last_price = BigDecimal::from(1);
        if params_price > last_price {
            panic!("params_price > last_price")
        }
    });
}

#[bench]
// #[ignore]
/*
test op_normal  ... bench: 172 ns/iter (+/- 7)
test op_ref     ... bench: 172 ns/iter (+/- 10)
结论：bigdecimal两边使用指针不仅会被clippy警告，而且速度更慢
*/
// FIXME 以下写法会被cargo clippy警告⚠️：needlessly taken reference of both operands
fn op_ref(bencher: &mut test::Bencher) {
    bencher.iter(|| {
        let params_price = BigDecimal::from(1);
        let last_price = BigDecimal::from(1);
        if &params_price > &last_price {
            panic!("params_price > last_price")
        }
    });
}
