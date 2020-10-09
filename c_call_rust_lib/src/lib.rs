use std::os::raw::{c_char, c_int};
use std::ffi::{CStr, CString};

#[no_mangle]
pub extern "C" fn p() {
    println!("call println in rust code");
}

#[no_mangle]
pub extern "C" fn hello(name: *const c_char) {
    assert!(!name.is_null());
    let c_str = unsafe {
        CStr::from_ptr(name)
    };
    println!("hello {:?}", c_str);
}

#[no_mangle]
pub extern "C" fn repeat_hi(times: c_int) -> *mut c_char {
    let mut res = String::new();
    res.extend(std::iter::repeat("hi").take(times as usize));
    CString::new(res).unwrap().into_raw()
}

#[no_mangle]
pub extern "C" fn free_cstr(str: *mut c_char) {
    // Rust的CStr交给C语言管理后，由于用的是堆内存，C语言并不会自动释放，所以还需要提供一个API去释放堆内存
    if str.is_null() {
        return;
    }
    unsafe {
        CString::from_raw(str);
    };
}
