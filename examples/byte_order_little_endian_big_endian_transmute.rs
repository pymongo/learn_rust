// 196608
const POSTGRES_PROTOCOL_VERSION_3: i32 = 0x00_03_00_00;
const RAW_BYTES: [u8; 4] = [0, 3, 0, 0];

/**
transmute, bytes order这些似乎在中std::intrinsics(内联函数)
结论: transmute只能用naive-endian的字节序，而且还是unsafe，建议用from_be_bytes代替transmute
*/
fn main() {
    let transmute_res = unsafe {
        // transmute default use os's naive endian, macOS/Linux default byte order is little-endian(LSB first, 小端序), LSB: Least Significant Bit
        // so RAW_BYTES's LSB->MSB is from left to right, LSB is RAW_BYTES[3]
        // naive-endian=little-endian: [0,3,0,0] => [0,0,3,0] = 3*256=768
        // CARGO_CFG_TARGET_ENDIAN: little
        std::mem::transmute::<[u8; 4], i32>(RAW_BYTES)
    };
    assert_ne!(transmute_res, POSTGRES_PROTOCOL_VERSION_3);
    assert_eq!(transmute_res, i32::from_ne_bytes(RAW_BYTES));
    // convert little-endian 768 to bigger-endian
    assert_eq!(i32::from_be(transmute_res), POSTGRES_PROTOCOL_VERSION_3);
    assert_eq!(i32::from_be_bytes(RAW_BYTES), POSTGRES_PROTOCOL_VERSION_3);
}

#[test]
fn test_i32_to_bool() {
    #[derive(Debug, Copy, Clone)]
    struct A {
        a: u16,
        b: u8,
        c: bool,
    }
    let a = {
        #[derive(Debug, Copy, Clone)]
        struct B {
            a: u16,
            b: u8,
            c: u8,
        }
        let b = B { a: 1, b: 1, c: 2 };
        unsafe { *(&b as *const B as *const A) }
    };
    // 尝试将 Some(a) 改为 a
    let some_a = Some(a);
    println!("a: {:#?}", a);
    println!("some_a: {:#?}", some_a);
    assert!(some_a.is_none());
}
