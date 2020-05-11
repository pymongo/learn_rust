#![feature(test)]
extern crate test; // 点IDE绿色的锤子编译时会报错，不过cargo build还是可以正常编译的
// benchmark为了测试最佳性能，都是release级编译，暂无--debug的选项

// cargo +nightly bench --bench benchmark_demo -- --nocapture
#[bench]
fn bench_reduce_add_10000(bencher: &mut test::Bencher) {
  bencher.iter(|| {
    // fold类似于其它语言的reduce
    (1..=10000).fold(0, |x, y| x + y);
    // 耗时<0ns
  });
}
