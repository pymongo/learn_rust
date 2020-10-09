use std::os::raw::{c_char, c_int};
use std::ffi::CStr;

#[no_mangle]
pub extern "C" fn p() {
    println!("call println in rust code");
}

#[no_mangle]
pub extern "C" fn hello(name: *const c_char) {
    let c_str = unsafe {
        CStr::from_ptr(name)
    };
    println!("hello {:?}", c_str);
}

#[no_mangle]
pub extern "C" fn repeat_hi()