#![feature(asm)]
//! 切记循环移位的搜索关键词是`bitewise rotate`或者汇编指令`ror/rol`，我以前老搜错成circle shift导致搜不到Rust的rotate_left API
//! 事实上汇编指令上就只有两种移位相关的指令: rol/ror和shl/shr，Rust的相应API命名上基本和汇编指令一致
//! https://www.aldeid.com/wiki/X86-assembly/Instructions/rol

fn main() {
    // 不推荐用乘法进行循环移位(主要是以前不知道有ror命令(忘记了)，用shl移位溢出时好像会清零)
    assert_eq!(1u8.checked_mul(2).unwrap_or(0x01), 1u8.rotate_left(1));
    // 另一种循环移位的方法请看
}