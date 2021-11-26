use std::ops::Generator;
use std::pin::Pin;

#[test]
fn main() {
    // 这部分生成器的源码展开后，会得到类似状态机的代码
    let mut func = || {
        // C++: std::this_thread::yield
        yield 1_i32;
        yield 2_i32;
        3_i32
    };
    for _ in 0..3 {
        let yield_res = Pin::new(&mut func).resume(());
        println!("{:?}", yield_res);
    }
}
