#![allow(incomplete_features)]
#![feature(const_generics, const_fn, const_in_array_repeat_expressions)]
use std::mem::MaybeUninit;

/**
当前(1.49.0)常量泛型的不足
1. T只能是整数、布尔值，也不允许使用引用(意味着不能用胖指针的字符串)
*/
struct Array<T, const N: usize> {
    items: [MaybeUninit<T>; N],
    length: usize,
}

trait ArrayLen {
    fn len(&self) -> usize;
}

impl<T, const N: usize> ArrayLen for Array<T, { N }> {
    #[inline]
    fn len(&self) -> usize {
        self.length
    }
}

/**
为什么常量泛型的长度会有花括号:
[https://internals.rust-lang.org/t/lang-team-minutes-const-generics/5090](https://internals.rust-lang.org/t/lang-team-minutes-const-generics/5090)
> Syntactically we may distinguish these expressions with braces
两个常量表达式的typechecking是T-lang团队的主要问题，目前{N+1}和{1+N}两个常量泛型长度会不一样
*/
impl<T: Copy, const N: usize> Array<T, { N }> {
    const fn new() -> Self {
        Self {
            items: [MaybeUninit::uninit(); N],
            length: 0,
        }
    }

    #[inline]
    const fn capacity(&self) -> usize {
        N
    }
}

fn main() {
    let visited: Array<bool, 8> = Array::new();
    dbg!(visited.len(), visited.capacity());
}
