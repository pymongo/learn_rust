use bigdecimal::BigDecimal;
use rust_decimal::Decimal;
// use rust_decimal::prelude::FromStr;

use std::mem::{align_of, align_of_val, size_of, size_of_val};
use std::str::FromStr;

fn main() {
    let rust_decimal_a = Decimal::from_str("3.1415926").unwrap();
    let bigdecimal_a = BigDecimal::from_str("3.1415926").unwrap();
    dbg!(size_of::<Decimal>());
    dbg!(size_of_val(&rust_decimal_a));
    dbg!(align_of::<Decimal>());
    dbg!(align_of_val(&rust_decimal_a));
    dbg!(size_of::<BigDecimal>());
    dbg!(size_of_val(&bigdecimal_a));
    dbg!(align_of::<BigDecimal>());
    dbg!(align_of_val(&bigdecimal_a));
    let json_data = serde_json::json!({ "key": bigdecimal_a });
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
