#![feature(generators, generator_trait)]
use std::future::Future;
use std::ops::Generator;
use std::pin::Pin;

fn main() {
    // 这部分生成器的源码展开后，会得到类似状态机的代码
    let mut func = || {
        // C++: std::this_thread::yield
        yield 1i32;
        yield 2i32;
        return 3i32;
    };
    for _ in 0..3 {
        let yield_res = Pin::new(&mut func).resume(());
        println!("{:?}", yield_res);
    }
}
