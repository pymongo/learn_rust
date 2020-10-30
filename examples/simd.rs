/*!
# CPU硬件相关的编程技术

## atomic和simd硬件API
```text
atomic和simd是Rust唯二两个「硬件API」，因为二者都需要考虑CPU架构x86或ARM是否支持
CPU硬件的API atomic性能要比操作系统层面的信号量内存壁垒API快得多
而simd则是更高效的利用CPU进行并行计算，提升运算速度
```

## false_sharing and cache_line_padded
```text
除了atomic和simd这两个要考虑CPU，多线程伪共享(False Sharing)及其解决方案「缓存行填充」也是CPU相关的编程技术
为了避免线程1和线程2的thread_local变量内存布局分布在同一个CPU缓存行上，造成线程1和线程2不能同时读取同一缓存行的数据带来的伪共享问题，
「就必须将多线程之间的数据隔离到不同的缓存行中」，从而提升并发性能
```

## 逐行遍历二维数组才能「命中CPU缓存」
*/

/**
62 |                 use crate::arch::x86_64::_mm_movemask_pi8;
   |                     ^^^^^^^^^^^^^^^^^^^^^----------------
   |                     |                    |
   |                     |                    help: a similar name exists in the module: `_mm_movemask_epi8`
   |                     no `_mm_movemask_pi8` in `arch::x86_64`
*/
fn main() {
    // FIXME faster库编译失败
    // use faster::*;
    //
    // let lots_of_10s = [-10i8; 3000].simd_iter(i8s(0))
    //     .simd_map(|v| v.abs())
    //     .scalar_collect();
    // assert_eq!(lots_of_10s, vec![10u8; 3000]);
}
