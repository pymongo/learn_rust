const fn gcd(x: u32, y: u32) -> u32 {
    let (mut a, mut b) = if x > y { (x, y) } else { (y, x) };
    let mut temp = a % b;
    while temp != 0 {
        a = b;
        b = temp;
        temp = a % b;
    }
    b
}

const GCD: u32 = gcd(18, 12);

/// 使用const fn提升了求解斐波那契数列的速度，也让特定斐波那契数的常量的可读性变得更好
const fn fib(n: u32) -> u32 {
    const fn helper(n: u32, a: u32, b: u32) -> u32 {
        return if n <= 2 { b } else { helper(n - 1, b, a + b) };
    }
    helper(n, 1, 1)
}

// 1 1 2 3 5
const FIB_5: u32 = fib(5);

fn main() {
    assert_eq!(GCD, 6);
    assert_eq!(FIB_5, 5);
}
