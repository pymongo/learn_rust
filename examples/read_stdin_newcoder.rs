//! [如何对ACM/牛客网/codeforces的stdin/stdout进行单元测试?](https://github.com/pymongo/leetcode-rust/blob/master/examples/codeforces_4a_watermelon.rs)
//! stdin/stdout的单元测试可以看我leetcode-rust仓库的codeforces题解
use std::io::BufRead;

/* 如果用Python解答，则可以用以下模板读取stdin
def parse_stdin():
    input_data = []
    for line in sys.stdin.readlines():
        input_data.append(line.rstrip('\n'))
    nums = [int(s) for s in input_data[0].split()]
    print(nums)
    print(input_data)
*/
fn main() {
    // 注意在IDEA上发送EOF的快捷键是cmd+d
    let mut input: Vec<String> = Vec::new();
    for line in std::io::stdin().lock().lines().flatten() {
        input.push(line);
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
