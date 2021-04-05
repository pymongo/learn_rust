# Learn Rust

## file descriptor

https://stackoverflow.com/questions/27665396/how-can-i-read-from-a-specific-raw-file-descriptor-in-rust

file descriptors index into a per-process file descriptor table maintained by the kernel
文件描述符在数据结构上是非负整数，每个进程都有各自的文件描述符表，其中0-2分别是stdin,stdout,stderr
文件描述符索引表类似虚拟内存，每个进程的fd索引表最终会映射到操作系统的fd表，类似每个进程的虚拟内存会最终映射到物理内存上，以此实现多个进程复用同一个块物理内存或fd
每个进程的fd表会存在`/proc/${PID}/fd`，例如PID=1是systemd，/proc/1/fd# file 11 => 11: symbolic link to /proc/1/mountinfo

// only work on linux
use std::os::unix::io::FromRawFd;
let mut f = unsafe { std::fs::File::from_raw_fd(3) };

---

## code snippets

### measure a function time cost

cargo bench or

```text
let now = std::time::Instant::now();
// call a function
println!("{:?}", now.elapsed());
```

### Rust没有函数重载，但是标准库IPv4的构造函数有类似函数重载的效果

```rust
// mock constructor overload in C++/java
struct Ip(u32);

impl From<u32> for Ip {
    fn from(val: u32) -> Self {
        Self(val)
    }
}

impl From<[u8; 4]> for Ip {
    fn from(val: [u8; 4]) -> Self {
        // bigger-endian
        Self(
            val[0] as u32 + (val[1] as u32)
                << 8 + (val[2] as u32)
                << 16 + (val[3] as u32)
                << 24,
        )
    }
}
```

### 一次迭代同时求出最大值和最小值

```rust
#[test]
fn iter_once_both_max_and_min() {
    let nums = vec![1i32, 2, 3, 4, 5];
    let (max, min) = nums.iter().fold((i32::MIN, i32::MAX), |(max, min), &x| {
        (max.max(x), min.min(x))
    });
    assert_eq!(max, *nums.iter().max().unwrap());
    assert_eq!(min, *nums.iter().min().unwrap());
}
```

---

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
