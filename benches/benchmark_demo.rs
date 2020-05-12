#![feature(test)]
extern crate test;
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
