//! [如何对ACM/牛客网/codeforces的stdin/stdout进行单元测试?](https://github.com/pymongo/leetcode-rust/blob/master/examples/codeforces_4a_watermelon.rs)
use std::io::BufRead;

fn main() {
    // 注意在IDEA上发送EOF的快捷键是cmd+d
    let mut input: Vec<String> = Vec::new();
    for line in std::io::stdin().lock().lines() {
        if let Ok(str) = line {
            input.push(str);
        }
    }
    let nums: Vec<i32> = input[0]
        .split_whitespace()
        .map(|x| x.parse::<i32>().unwrap())
        .collect();
    println!("{:?}", nums);
    // use std::io::Write; // 如果要求输出末尾不能换行，则用print!宏加stdout().flush()
    // print!("output"); std::io::stdout().flush().unwrap();
    dbg!(&input);
}
