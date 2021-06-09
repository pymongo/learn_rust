//! 切记循环移位的搜索关键词是`bitewise rotate`或者汇编指令`ror/rol`，我以前老搜错成circle shift导致搜不到Rust的rotate_left API
//! 事实上汇编指令上就只有两种移位相关的指令: rol/ror和shl/shr，Rust的相应API命名上基本和汇编指令一致
//! https://www.aldeid.com/wiki/X86-assembly/Instructions/rol
#![feature(asm)]

fn main() {
    let mut register: usize = 1; // default use rax register
                                 // asm ROL command is same as Rust rotate_left API
    unsafe {
        asm!(
        "rol {0}, 1",
        inout(reg) register => register, // or inout(reg) register,
        );
    }
    assert_eq!(register, 1usize.rotate_left(1));
}

/// https://blog.rust-lang.org/inside-rust/2020/06/08/new-inline-asm.html
#[test]
fn test_asm_syscall_write() {
    let buf = "Hello from asm!\n";
    let ret: i32;
    unsafe {
        asm!(
        "syscall",
        in("rax") 1, // syscall number
        in("rdi") 1, // fd (stdout)
        in("rsi") buf.as_ptr(),
        in("rdx") buf.len(),
        out("rcx") _, // clobbered by syscalls
        out("r11") _, // clobbered by syscalls
        lateout("rax") ret,
        );
    }
    println!("write returned: {}", ret);
}

#[test]
fn test_asm_popcnt_instruction() {
    let input_value: usize = 3;
    let popcnt_output: usize;
    unsafe {
        asm!(
        "popcnt {popcnt_output}, {input_value}",
        input_value = in(reg) input_value,
        popcnt_output = out(reg) popcnt_output,
        );
    }
    dbg!(popcnt_output);
}
