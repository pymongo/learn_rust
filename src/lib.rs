// #![no_std]
#[test]
fn t() {
    let mut n = 1u8;
    for _ in 0..16 {
        dbg!(n);
        n = n.checked_mul(2).unwrap_or(1);
    }
}
