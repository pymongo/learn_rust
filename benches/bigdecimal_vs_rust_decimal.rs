#![feature(test)]
extern crate test;
// cargo +nightly bench --bench bigdecimal

use rust_decimal::Decimal;
use rust_decimal::RoundingStrategy::RoundHalfUp;
// 两个库的Zero, One Trait都是用的同一个crate
use bigdecimal::{BigDecimal, Zero, One, Signed, ToPrimitive};
use num_bigint::BigInt;
use serde::{Deserialize, Serialize};
use std::str::FromStr;

#[bench]
fn bigdecimal_mul(bencher: &mut test::Bencher) {
    bencher.iter(|| {
        let price = BigDecimal::from_str("1.1").unwrap();
        let volume = BigDecimal::from(1);
        let _total = &price * &volume;
    });
}

/* 乘法 decimal的性能2.5倍以上
test bigdecimal_mul ... bench:         566 ns/iter (+/- 590)
test decimal_mul    ... bench:         193 ns/iter (+/- 67)
*/
#[bench]
fn decimal_mul(bencher: &mut test::Bencher) {
    bencher.iter(|| {
        let price = Decimal::from_str("1.1").unwrap();
        let volume = Decimal::from(1);
        // Decimal这个库更简单方便，乘法的lhs和rhs不必写成指针形式，
        // 就算写成指针格式(无论指针怎么加)，运算速度都一样，而BigDecimal如果乘法的左右两边(lhs和rhs)不是指针，性能慢很多
        let _total = price * volume;
    });
}

#[bench]
fn bigdecimal_div(bencher: &mut test::Bencher) {
    bencher.iter(|| {
        let a = BigDecimal::from_str("9.86960406437476").unwrap();
        let b = BigDecimal::from_str("3.1415926").unwrap();
        let _ = &a / &b;
    });
}

/* 除法: 两个库打平。decimal的作者承认除法为了算准很耗费时间，很难优化
test bigdecimal_div       ... bench:       1,199 ns/iter (+/- 289)
test decimal_div          ... bench:       1,166 ns/iter (+/- 262)
*/
#[bench]
fn decimal_div(bencher: &mut test::Bencher) {
    bencher.iter(|| {
        let a = Decimal::from_str("9.86960406437476").unwrap();
        let b = Decimal::from_str("3.1415926").unwrap();
        let _ = a / b;
    });
}

const NUMS: [&str; 5] = ["1.12", "2.41", "3.1415926", "7.65", "8.12"];

#[bench]
fn bigdecimal_cmp(bencher: &mut test::Bencher) {
    bencher.iter(|| {
        let mut nums: Vec<BigDecimal> = Vec::with_capacity(5);
        for num in &NUMS {
            nums.push(BigDecimal::from_str(num).unwrap());
        }
        let target = BigDecimal::from_str("7.65").unwrap();
        let _ = nums.binary_search(&target).unwrap();
    });
}

/* 大小比较: decimal性能翻倍
test bigdecimal_cpm       ... bench:       2,647 ns/iter (+/- 361)
test decimal_cmp          ... bench:       1,486 ns/iter (+/- 384)
*/
#[bench]
fn decimal_cmp(bencher: &mut test::Bencher) {
    bencher.iter(|| {
        let mut nums: Vec<Decimal> = Vec::with_capacity(5);
        for num in &NUMS {
            nums.push(Decimal::from_str(num).unwrap());
        }
        let target = Decimal::from_str("7.65").unwrap();
        let _ = nums.binary_search(&target).unwrap();
    });
}

#[bench]
fn bigdecimal_construct(bencher: &mut test::Bencher) {
    bencher.iter(|| {
        let _ = BigDecimal::from_str("-3.1415926").unwrap();
        let _ = BigDecimal::zero();
        let _ = BigDecimal::one();
        let _ = BigDecimal::from(2);
    });
}

/* 构造方面, decimal性能好50%
test bigdecimal_construct ... bench:         501 ns/iter (+/- 37)
test decimal_construct    ... bench:         345 ns/iter (+/- 84)
*/
#[bench]
fn decimal_construct(bencher: &mut test::Bencher) {
    bencher.iter(|| {
        let _ = Decimal::from_str("-3.1415926").unwrap();
        let _ = Decimal::zero();
        let _ = Decimal::one();
        let _ = Decimal::from(2);
    });
}

#[bench]
fn bigdecimal_get_sign(bencher: &mut test::Bencher) {
    bencher.iter(|| {
        let a = BigDecimal::from_str("-3.1415926").unwrap();
        let b = BigDecimal::from(2);
        assert!(a.is_negative());
        assert!(b.is_positive());
    });
}

/* 获取符号: decimal性能好50%
test bigdecimal_get_sign  ... bench:         448 ns/iter (+/- 27)
test decimal_get_sign     ... bench:         306 ns/iter (+/- 90)
*/
#[bench]
fn decimal_get_sign(bencher: &mut test::Bencher) {
    bencher.iter(|| {
        let a = Decimal::from_str("-3.1415926").unwrap();
        let b = Decimal::from(2);
        assert!(a.is_sign_negative());
        assert!(b.is_sign_positive());
    });
}

#[bench]
fn bigdecimal_get_scale(bencher: &mut test::Bencher) {
    bencher.iter(|| {
        let a = BigDecimal::from_str("3.1415926").unwrap();
        let b = BigDecimal::from(2);
        assert_eq!(a.as_bigint_and_exponent().1, 7);
        assert!(b.is_integer());
    });
}

/* 获取精度: decimal性能好70%
test bigdecimal_get_scale ... bench:         558 ns/iter (+/- 41)
test decimal_get_scale    ... bench:         321 ns/iter (+/- 42)
*/
#[bench]
fn decimal_get_scale(bencher: &mut test::Bencher) {
    bencher.iter(|| {
        let a = Decimal::from_str("3.1415926").unwrap();
        let b = Decimal::from(2);
        assert_eq!(a.scale(), 7);
        assert_eq!(b.scale(), 0);
    });
}

fn round(decimal: &BigDecimal, round_digits: i64) -> BigDecimal {
    let (bigint, decimal_part_digits) = decimal.as_bigint_and_exponent();
    let need_to_round_digits = decimal_part_digits - round_digits;
    if round_digits >= 0 && need_to_round_digits <= 0 {
        return decimal.clone();
    }

    let mut number = bigint.to_i128().unwrap();
    if number < 0 {
        number = -number;
    }
    for _ in 0..(need_to_round_digits - 1) {
        number /= 10;
    }
    let digit = number % 10;

    if digit <= 4 {
        decimal.with_scale(round_digits)
    } else if bigint.is_negative() {
        decimal.with_scale(round_digits) - BigDecimal::new(BigInt::from(1), round_digits)
    } else {
        decimal.with_scale(round_digits) + BigDecimal::new(BigInt::from(1), round_digits)
    }
}

const ROUND_TEST_CASES: [(&str, u32, &str); 14] = [
    ("0.085", 2, "0.09"),
    ("1.45", 1, "1.5"),
    ("1.444445", 1, "1.4"),
    ("1.44", 1, "1.4"),
    ("0.444", 2, "0.44"),
    ("0.0045", 2, "0.00"),
    ("-1.555", 2, "-1.56"),
    ("-1.555", 99, "-1.555"),
    ("5.5", 0, "6"),
    // ("-1", -1, "0"),
    // ("5", -1, "10"),
    // ("44", -1, "40"),
    // ("44", -99, "0"),
    ("1.4499999999", 1, "1.4"),
    ("-1.4499999999", 1, "-1.4"),
    ("1.449999999", 1, "1.4"),
    ("-9999.444455556666", 10, "-9999.4444555567"),
    (
        "-12345678987654321.123456789",
        8,
        "-12345678987654321.12345679",
    ),
];

#[bench]
fn bigdecimal_round(bencher: &mut test::Bencher) {
    bencher.iter(|| {
        for &(input, round_digit, output) in &ROUND_TEST_CASES {
            let input = BigDecimal::from_str(input).unwrap();
            let output = BigDecimal::from_str(output).unwrap();
            assert_eq!(round(&input, round_digit as i64), output);
        }
    });
}

/* round方面: decimal性能翻倍，而且提供多种round的策略
test bigdecimal_round ... bench:      17,498 ns/iter (+/- 3,707)
test decimal_round    ... bench:       8,747 ns/iter (+/- 1,475)
*/
#[bench]
fn decimal_round(bencher: &mut test::Bencher) {
    bencher.iter(|| {
        for &(input, round_digit, output) in &ROUND_TEST_CASES {
            let input = Decimal::from_str(input).unwrap();
            let output = Decimal::from_str(output).unwrap();
            assert_eq!(input.round_dp_with_strategy(round_digit, RoundHalfUp), output);
        }
    });
}

#[derive(Serialize, Deserialize)]
struct BigDecimalForm {
    data: BigDecimal
}

#[derive(Serialize, Deserialize)]
struct DecimalForm {
    data: Decimal
}

#[bench]
fn bigdecimal_serialize(bencher: &mut test::Bencher) {
    bencher.iter(|| {
        let raw_data = BigDecimalForm { data: BigDecimal::from_str("3.1415926").unwrap() };
        let serialized_string = serde_json::to_string(&raw_data).unwrap();
        let deserialized_data: BigDecimalForm = serde_json::from_str(&serialized_string).unwrap();
        assert!(deserialized_data.data.eq(&raw_data.data))
    });
}


/* 序列化和反序列化两个库差距不大，bigdecimal更稳定
test bigdecimal_serialize ... bench:       1,454 ns/iter (+/- 72)
test decimal_serialize    ... bench:       1,362 ns/iter (+/- 225)
*/
#[bench]
fn decimal_serialize(bencher: &mut test::Bencher) {
    bencher.iter(|| {
        let raw_data = DecimalForm { data: Decimal::from_str("3.1415926").unwrap() };
        let serialized_string = serde_json::to_string(&raw_data).unwrap();
        let deserialized_data: DecimalForm = serde_json::from_str(&serialized_string).unwrap();
        assert!(deserialized_data.data.eq(&raw_data.data))
    });
}
