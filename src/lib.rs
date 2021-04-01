pub fn random() -> i32 {
    #[allow(non_camel_case_types)]
    type time_t = i64; // 根据libc源码中time_t的类型定义
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
