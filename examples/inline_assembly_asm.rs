#![feature(asm)]

fn main() {
    unsafe {
        asm!("NOP");
    }
    println!("after NOP");
}
