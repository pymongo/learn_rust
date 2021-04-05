fn main() {
    use std::mem::{align_of, size_of, size_of_val};
    #[derive(Clone)]
    struct HeavySize([u32; 25]);
    // Rust enum is tagged_union in C, need one or more usize to knowns which kind of value it holds
    assert_eq!(size_of::<Option<HeavySize>>(), 100 + 4);
    assert_eq!(align_of::<Option<HeavySize>>(), 4); // 8
    let heavy_size = HeavySize([0; 25]);
    assert_eq!(size_of_val(&heavy_size), 100);

    // if the size of T in Option<T> is too large, recommend to use Option<Box<T>>: 1.方便内存对齐 2.减小体积
    // 但是我认为用Option<Box<T>>为了去减少函数间传递对象的体积，还不如用指针，因为项目里ApiError这个enum成员是各种各样的T都有的
    // 不管怎么说还是要警惕Option<T>当T体积很大时导致变量Option<T>的内存体积也很大
    assert_eq!(size_of_val(&Some(heavy_size.clone())), 104);
    assert_eq!(size_of_val(&Box::new(Some(heavy_size.clone()))), 8);
}
