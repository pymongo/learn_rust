#![feature(test)]
extern crate test;
// cargo +nightly bench --bench bigdecimal

use rust_decimal::Decimal;
use rust_decimal::RoundingStrategy::RoundHalfUp;
use bigdecimal::BigDecimal;
use bigdecimal::ToPrimitive;
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

/*
test bigdecimal_mul ... bench:         566 ns/iter (+/- 590)
test decimal_mul    ... bench:         193 ns/iter (+/- 67)
*/
#[bench]
fn decimal_mul(bencher: &mut test::Bencher) {
    bencher.iter(|| {
        let price = Decimal::from_str("1.1").unwrap();
        let volume = Decimal::from(1);
        let _total = &price * &volume;
    });
}

fn round(decimal: &BigDecimal, round_digits: i64) -> BigDecimal {
    use bigdecimal::Signed;

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

/*
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

#[bench]
fn decimal_serialize(bencher: &mut test::Bencher) {
    bencher.iter(|| {
        let raw_data = DecimalForm { data: Decimal::from_str("3.1415926").unwrap() };
        let serialized_string = serde_json::to_string(&raw_data).unwrap();
        let deserialized_data: DecimalForm = serde_json::from_str(&serialized_string).unwrap();
        assert!(deserialized_data.data.eq(&raw_data.data))
    });
}
