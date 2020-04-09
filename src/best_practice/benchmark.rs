/*
#![feature(test)] // 必须放在main.rs的第1-2行
extern crate test;
*/

#[cfg(test)]
#[bench]
fn bench_reduceadd_1000(bencher: &mut test::Bencher) {
  bencher.iter(|| {
    // 运行1万次
    //   let n = test::black_box(1000);
    // (1..=n).fold(0, |x, y| x ^ y);
    // fold类似于其它语言的reduce
    (1..=1000).fold(0, |x, y| x + y);
  });
}

