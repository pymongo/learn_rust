/*!
https://stackoverflow.com/questions/38315383/whats-the-rust-idiom-to-define-a-field-pointing-to-a-c-opaque-pointer
https://www.educative.io/edpresso/how-to-write-regular-expressions-in-c
[FFI绑定教程2](https://mp.weixin.qq.com/s?__biz=MzI1MjAzNDI1MA==&mid=2648211118&idx=1&sn=f1bccf7e10537ff3a828703579b1df3f&chksm=f1c5330bc6b2ba1d0a4739f03838ef9a16272e2aa458ad8f58da65a35a7150c85e6b1ef110ae&cur_album_id=1319566712852873217&scene=189#rd)
*/
use std::os::raw::{c_char, c_int};

#[allow(non_camel_case_types)]
#[repr(C)]
struct regex_t {
    _private: [u8; 0],
}

#[allow(non_camel_case_types)]
#[repr(C)]
struct regmatch_t {
    /// start of match
    rm_so: isize,
    /// end of match
    rm_eo: isize,
}

extern "C" {
    /// int regcomp(regex_t *preg, const char *regex, int cflags);
    fn regcomp(preg: *mut regex_t, regex: *const c_char, cflags: c_int) -> c_int;
    /// int regexec(const regex_t *preg, const char *string, size_t nmatch, regmatch_t pmatch[], int eflags);
    fn regexec(
        preg: *const regex_t,
        string: *const c_char,
        nmatch: usize,
        pmatch: *mut regmatch_t,
        eflags: c_int,
    ) -> c_int;
    /// void regfree(regex_t *preg);
    fn regfree(preg: *mut regex_t);
}

fn main() {
    unsafe {
        let mut preg: regex_t = std::mem::zeroed();
        let regcomp_res = regcomp(&mut preg as *mut _, b"ice\0".as_ptr() as *const _, 0);
        assert_eq!(regcomp_res, 0);
        let mut match_position: regmatch_t = std::mem::zeroed();
        match regexec(
            &mut preg as *const _,
            b"icecream\0".as_ptr() as *const _,
            0usize,
            // std::ptr::null_mut(),
            &mut match_position as *mut regmatch_t,
            0i32,
        ) {
            0 => println!("Pattern found."),
            1 => println!("Pattern not found."),
            _ => println!("An error occurred."),
        }
        dbg!(match_position.rm_so, match_position.rm_eo);
        regfree(&mut preg as *mut _);
    }
}
