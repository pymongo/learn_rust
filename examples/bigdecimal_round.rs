/*
由于我设计的round的小算法已被[merge](https://github.com/akubera/bigdecimal-rs/pull/66)
所以这里只是结合Python做一下单元测试

对撮合特定情况进行优化的round()，例如入参没有负数
思路，遍历过程中只需判断到小数点后第{digits}位时是否出现「五入」的进位情况
[四舍的情况]: 如果没出现「五入」，则原封不动地返回入参的Decimal.with_scale(digits)
[五入的情况]: 如果出现了，就返回原Decimal.with_scale(digits)+(1/10**digits)
MySQL、python、round的round只跟倒数第二位有关

例如round(1.114, 2)
首先要根据bigdecimal内部的小数点位置和round找到需要round的那位在哪: need_to_round_digits = decimal_part_digits - round_digits
例如1.114的decimal_part是3，round_digit是2，所以我需要round的digit是1114从右往左数的3-2位
然后 for _ in 0..0 {num /= 10}，结果num还是1114，我们就取到的digit就是4
1. 如果取到的digit是4，那么啥也不干直接将输入的bigdecimal通过with_scale API截断成2位
2. 如果取到的digit大于5，
2.1 如果需要五入而且是负数，则截断成2位后，减去0.01，例如-1.115 => -1.12
2.2 如果需要五入是正数，则截断2位后，加上0.01，例如 1.785 => 1.79
*/
use bigdecimal::BigDecimal;
//use rand::Rng;
use std::process::Command;
use std::str::FromStr;

fn main() {}

#[cfg(not)]
fn test_main() {
    let mut rng = rand::thread_rng();
    // 注意python3的`round(0.2823398503991065, 15)`结果是错的，所以round_digits的范围不要太大, round超过15python容易报错
    for _ in 0..200 {
        let random_float = rng.gen::<f64>();
        let random_digits = rng.gen_range(-14, 14);
        let python_expr = format!("print(round({}, {}))", random_float, random_digits);
        println!("{}", python_expr);
        let output = Command::new("python3")
            .arg("-c")
            .arg(&python_expr)
            .output()
            .unwrap();
        let input_decimal = BigDecimal::from_str(&random_float.to_string()).unwrap();
        let round_result = input_decimal.round(random_digits);
        let python_output = String::from_utf8(output.stdout).unwrap();
        // use trim API to remove '\n' at the end of python print
        let python_round_result = BigDecimal::from_str(python_output.trim()).unwrap();
        assert_eq!(round_result, python_round_result);
    }
}
