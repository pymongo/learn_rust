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
