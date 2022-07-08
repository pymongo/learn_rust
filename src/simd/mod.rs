//! simd 的应用: crc 循环冗余校验算法(也可作为简易哈希算法)，哈希运算

#[test]
fn vector_multiple() {
    let vector = std::simd::Simd::from_array([2_i32]);
    println!("{:?}", vector * std::simd::Simd::from_array([2_i32]));
}

/// CRC 循环冗余校验码 算法
/// ping/icmp 的 checksum: u16 就是用 CRC 算出来的
#[test]
fn simd_crc() {
    unsafe {
        dbg!(std::arch::x86_64::_mm_crc32_u16(0, 1));
    }
}
