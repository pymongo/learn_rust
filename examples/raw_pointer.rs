struct Nums(Vec<i32>);

impl Nums {
    // 基本上参照Vec的insert源码，主要是理解原始指针在操作数组上的好处
    fn insert(&mut self, i: usize, val: i32) {
        let len = self.0.len();
        assert!(i < len);
        // 先给数组扩容
        self.0.reserve(1);
        unsafe {
            {
                let p = self.0.as_mut_ptr().add(i);
                // 将要插入位置往右的所有元素都右移一格
                std::ptr::copy(p, p.offset(1), len - i);
                // 往空出来的位置上插入新值
                std::ptr::write(p, val);
            }
            self.0.set_len(len + 1);
        }
    }
}

fn main() {
    let mut nums = Nums(vec![0, 1, 2]);
    nums.insert(1, -1);
    println!("{:?}", nums.0);
    unsafe {
        let s = "Rust";
        let s_prt = s.as_ptr();
        // 因为.read()不会转移所有权，所以读的过程中值有可能会修改，所以read方法是unsafe的
        dbg!(s_prt.read() as char);
        let nums1 = vec![1, 2, 3];
        let nums_ptr_1: *const i32 = nums1.as_ptr();
        let nums2 = vec![1, 2, 3];
        let _nums_ptr_2 = &nums2 as *const Vec<i32>;
        assert_eq!(nums_ptr_1.read(), 1);
        // 下面这行会导致程序异常中止: Process finished with exit code 134 (interrupted by signal 6: SIGABRT)
        // println!("{:?}", _nums_ptr_2.read());
    }
}
