/**
free(): double free detected in tcache 2
timeout: the monitored command dumped core
/playground/tools/entrypoint.sh: line 11:     7 Aborted                 timeout --signal=KILL ${timeout} "$@"
*/
fn main() {
    let str = "rust".to_string();
    let ptr = &str as *const String as usize;
    println!("the smart pointer of str = {:p}", &str);
    println(ptr);
    // double free: Process finished with exit code 134 (interrupted by signal 6: SIGABRT)
    println(ptr);
}

fn println(ptr: usize) {
    unsafe {
        let deref_ptr = std::ptr::read(ptr as *const String);
        println!("{}", deref_ptr);
    }
    // 释放了指向堆u8序列
}
