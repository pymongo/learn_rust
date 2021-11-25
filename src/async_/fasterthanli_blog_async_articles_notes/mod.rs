/*!
- [x] 1. https://fasterthanli.me/articles/getting-in-and-out-of-trouble-with-rust-futures
    - async_trait(?Send) 编译报错的 debug
- [ ] 2. https://fasterthanli.me/articles/surviving-rust-async-interfaces
    - 抛出问题1: 用 sha3 库/算法哈希文件比 Linux 的 openssl dgst -sha3-256 wine-5.0.2.tar.xz 慢很多
    - 解决思路1: 用 async-std 的异步 IO 能否加速？
    - 抛出问题2: Future 引用的生命周期要求 static
    - 解决思路2: 手写状态机
    文章后半部分代码太长解决 Future 各种编译报错的手造状态机太复杂了一时间我也没能看懂
- [ ] 3. https://fasterthanli.me/articles/pin-and-suffering
    - 一开始是 5-6 个手工实现 Future 的调度
*/
/*!
## 2. async speed up sha3 hash
### (gdb) break pthread_create
观察 pthread_create 的调用栈
*/
mod custom_impl_future;
mod std_api;
mod trace_async_read;
