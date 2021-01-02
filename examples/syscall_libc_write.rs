/**
#include <unistd.h> // write
#include <string.h> // strlen

int main() {
    const char* text = "Hello World\n";
    ssize_t write_len = write(1, text, strlen(text));
    return 0;
}
*/
#[allow(non_camel_case_types)]
type size_t = usize;
#[allow(non_camel_case_types)]
type ssize_t = isize;

extern "C" {
    fn write(fd: std::os::raw::c_int, buf: *const std::ffi::c_void, count: size_t) -> ssize_t;
}

fn print(bytes: &[u8]) {
    const STDOUT: i32 = 1;
    let bytes_c_void_ptr = bytes.as_ptr() as *const core::ffi::c_void;
    unsafe {
        // system call `write` in #include <unistd.h>
        // ssize_t write(int fd, const void *buf, size_t count);
        let write_len = write(STDOUT, bytes_c_void_ptr, bytes.len());
        assert_eq!(write_len, bytes.len() as isize);
    }
}

fn main() {
    print(b"Hello World!\n\0");
}
