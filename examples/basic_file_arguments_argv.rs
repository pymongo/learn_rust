#[allow(dead_code)]
pub fn run() {
    // 获取命令行参数
    let args: Vec<String> = std::env::args().collect();

    for arg in &args {
        println!("{}", arg);
    }
}
