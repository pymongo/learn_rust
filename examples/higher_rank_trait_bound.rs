
trait A<T> {
    fn a(&self, value: T);
}

// for<'a> Higher-Rank Trait Bounds
impl<'a, T: std::fmt::Debug> A<T> for &'a usize {
    fn a(&self, value: T) {
        // 这个函数没有返回引用，是安全的，没有悬垂指针的安全问题
        println!("{:?}", value);
    }
}

#[cfg(not)]
/// num not live long enough
/// 编译器不会将a()和f()两个函数联系起来分析生命周期，所以这里认为 &num会是悬垂指针
fn f<'a>(b: Box<dyn A<&'a usize>>) {
    let num: usize = 10;
    b.a(&num);
}

/// late bound 延后到b.a()调用时再去判断生命周期
fn f(b: Box<dyn for<'b> A<&'b usize>>) {
    let num: usize = 10;
    b.a(&num);
}

fn main() {
    let a = Box::new(&usize::MIN);
    f(a);
}

