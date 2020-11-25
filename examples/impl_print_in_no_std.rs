// #![no_std]

fn print(bytes: &[u8]) {
    const STDOUT: i32 = 1;
    let bytes_c_void_ptr = bytes.as_ptr() as *const core::ffi::c_void;
    unsafe {
        // system call `write` in #include <unistd.h>
        // ssize_t write(int fd, const void *buf, size_t count);
        let write_len = libc::write(STDOUT, bytes_c_void_ptr, bytes.len());
        assert_eq!(write_len, bytes.len() as isize);
    }
}

// #[panic_handler]
// fn my_panic(_info: &core::panic::PanicInfo) -> ! {
//     loop {}
// }

fn main() {
    print(b"Hello World!\n\0");
}
