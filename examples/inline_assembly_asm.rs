#![feature(asm)]

// cargo +nightly run --example inline_assembly
fn main() {
    unsafe {
        // asm!("NOP");
        llvm_asm!("NOP");
    }
    println!("after NOP");
}
