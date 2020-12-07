use std::ffi::{CStr, CString};
use std::os::raw::{c_char, c_int, c_uint};

#[no_mangle]
pub extern "C" fn hello(name: *const c_char) {
    assert!(!name.is_null());
    let c_str = unsafe { CStr::from_ptr(name) };
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
    // 在Rust函数内创建的堆内存变量，C语言并不会自动释放，所以还需要在Rust侧提供一个API去释放堆内存
    if str.is_null() {
        return;
    }
    unsafe {
        CString::from_raw(str);
    };
}

#[no_mangle]
pub extern "C" fn sum_of_positive(arr: *const c_int, len: c_uint) -> c_int {
    if arr.is_null() {
        return c_int::from(-1);
    }
    let nums = unsafe { std::slice::from_raw_parts(arr, len as usize) };
    let sum: i32 = nums.iter().filter(|&x| x.is_positive()).sum();
    c_int::from(sum)
}

#[derive(Debug)]
#[repr(C)]
pub struct Point {
    x: i32,
    y: i32,
}

#[no_mangle]
pub extern "C" fn print_point(point: Point) {
    dbg!(point);
}

use std::collections::HashMap;
pub struct Map(HashMap<c_int, c_int>);

impl Map {
    fn new() -> Self {
        Self(HashMap::new())
    }

    fn insert(&mut self, key: c_int, value: c_int) {
        self.0.insert(key, value);
    }

    fn get(&self, key: c_int) -> c_int {
        return if let Some(&val) = self.0.get(&key) {
            val
        } else {
            c_int::from(-1)
        };
    }
}

// Rust:Box<T> -> C:opaque type(不透明数据类型)
#[no_mangle]
pub extern "C" fn map_new() -> *mut Map {
    // Map::new() as *mut Map
    Box::into_raw(Box::new(Map::new()))
}

#[no_mangle]
pub extern "C" fn map_insert(map: *mut Map, k: c_int, v: c_int) {
    if map.is_null() {
        return;
    }
    let map_deref = unsafe { &mut *map };
    map_deref.insert(k, v);
}

#[no_mangle]
pub extern "C" fn map_get(map: *const Map, k: c_int) -> c_int {
    if map.is_null() {
        return c_int::from(-1);
    }
    let map_deref = unsafe { &*map };
    map_deref.get(k)
}

#[no_mangle]
pub extern "C" fn map_free(map: *mut Map) {
    if map.is_null() {
        return;
    }
    Box::from(map);
}
