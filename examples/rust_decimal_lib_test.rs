use rust_decimal::Decimal;
use bigdecimal::BigDecimal;
// use rust_decimal::prelude::FromStr;
use std::str::FromStr;
use std::mem::size_of_val;

fn main() {
    let rust_decimal_a = Decimal::from_str("3.1415926").unwrap();
    let bigdecimal_a = BigDecimal::from_str("3.1415926").unwrap();
    assert_eq!(size_of_val(&rust_decimal_a), 16);
    assert_eq!(size_of_val(&bigdecimal_a), 40);
}