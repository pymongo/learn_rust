# Learn Rust

## code snippets

### measure a function time cost

cargo bench or

```text
let now = std::time::Instant::now();
// call a function
println!("{:?}", now.elapsed());
```

## CPU硬件相关的编程技术

### simd和atomic
```text
atomic和simd是Rust两个硬件API」，因为二者都需要考虑CPU架构x86或ARM是否支持
CPU硬件的API atomic性能要比操作系统层面的信号量内存壁垒API快得多
而simd则是更高效的利用CPU进行并行计算，提升运算速度
```

simd的代码难理解难编译运行，故没有代码演示

### false_sharing and cache_line_padded
```text
除了atomic和simd这两个要考虑CPU，多线程伪共享(False Sharing)及其解决方案「缓存行填充」也是CPU相关的编程技术
为了避免线程1和线程2的thread_local变量内存布局分布在同一个CPU缓存行上，造成线程1和线程2不能同时读取同一缓存行的数据带来的伪共享问题，
「就必须将多线程之间的数据隔离到不同的缓存行中」，从而提升并发性能
```

逐行遍历二维数组才能「命中CPU缓存」
