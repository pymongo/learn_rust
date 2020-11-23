#![feature(asm)]

fn main() {
    let mut register: usize = 1;
    unsafe {
        asm!(
        "rol {0}, 1",
        inout(reg) register => register, // or inout(reg) register,
        );
    }
    assert_eq!(register, 1usize.rotate_left(1));
}
