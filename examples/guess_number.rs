use std::io::BufRead;

/**
一个动态链接库(dynamic linking library, 也叫so文件)可以对应一个或多个头文件，例如boost库
例如 libc.so 就有 time.h, stdlib.h 等多个头文件
TODO 为什么 openssl库 既有libcrypto.so也有libssl.so
可以通过`nm`命令工具获知某个so文件内有没有某个函数
```text
[w@w-manjaro ~]$ nm -D /usr/lib/libc.so.6 | grep gmtime
00000000000bb480 T gmtime@@GLIBC_2.2.5
00000000000bb460 T __gmtime_r@@GLIBC_2.2.5
00000000000bb460 W gmtime_r@@GLIBC_2.2.5
```
*/
fn random() -> i32 {
    #[allow(non_camel_case_types)]
    type time_t = i64; // 根据libc源码中time_t的类型定义
    #[link(name = "c")]
    extern "C" {
        /// https://en.cppreference.com/w/cpp/chrono/c/time
        fn time(arg: *mut time_t) -> time_t;
        /// https://www.cplusplus.com/reference/cstdlib/rand/
        fn rand() -> i32;
        fn srand(seed: u32);
    }

    use std::sync::Once;
    static INIT_RAND_SEED: Once = Once::new();
    INIT_RAND_SEED.call_once(|| unsafe {
        let mut current_timestamp: time_t = std::mem::zeroed();
        time(&mut current_timestamp as *mut time_t);
        srand(current_timestamp as u32);
    });

    unsafe { rand() }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Guess the number(1-100)!");
    let secret_number = random() as u32 % 100;
    loop {
        println!("\nPlease input your guess:");
        let mut guess_stdin_buf = String::new();
        std::io::stdin().lock().read_line(&mut guess_stdin_buf)?;
        let guess = guess_stdin_buf.trim().parse::<u32>()?;
        match guess.cmp(&secret_number) {
            std::cmp::Ordering::Less => println!("Too small!"),
            std::cmp::Ordering::Greater => println!("Too big!"),
            std::cmp::Ordering::Equal => break,
        }
    }
    println!("You guess is correct. You win!");
    Ok(())
}
