#[link(name="cpp_lib", kind="static")]
extern {
    // `extern` block uses type `[i32]`, which is not FFI-safe
    fn cpp_sort(nums: &[i32; 5], n: u32);
}

fn main() {
    let nums = [1, 3, 2, 5, 4];
    unsafe {
        cpp_sort(&nums, nums.len() as u32);
    }
    println!("{:?}", nums);
}