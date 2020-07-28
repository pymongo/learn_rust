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
    let json_data = serde_json::json!({"key": bigdecimal_a});
    // 默认是序列化为String，可以设置序列化为float
    dbg!(json_data);
    let a = Decimal::from_str("9.86960406437476").unwrap();
    let b = Decimal::from_str("3.1415926").unwrap();
    dbg!(a / b);
    /*
    {
        "key": String(
            "3.1415926",
        ),
    }
    */
}