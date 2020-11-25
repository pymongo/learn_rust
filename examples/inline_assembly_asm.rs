//! 切记循环移位的搜索关键词是`bitewise rotate`或者汇编指令`ror/rol`，我以前老搜错成circle shift导致搜不到Rust的rotate_left API
//! 事实上汇编指令上就只有两种移位相关的指令: rol/ror和shl/shr，Rust的相应API命名上基本和汇编指令一致
//! https://www.aldeid.com/wiki/X86-assembly/Instructions/rol
#![feature(asm)]

fn main() {
    let mut register: usize = 1;
    // asm ROL command is same as Rust rotate_left API
    unsafe {
        asm!(
        "rol {0}, 1",
        inout(reg) register => register, // or inout(reg) register,
        );
    }
    assert_eq!(register, 1usize.rotate_left(1));
    assert_ne!(f32::NAN, f32::NAN);
}
