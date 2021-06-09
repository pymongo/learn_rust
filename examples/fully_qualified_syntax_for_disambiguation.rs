struct S;

trait A {
    fn test(&self) {
        println!("Calling A's instance method `test`");
    }
}

trait B {
    fn test(&self) {
        println!("Calling B's instance method `test`");
    }
}

impl A for S {}

impl B for S {}

fn main() {
    let s = S;
    // multiple `test` found, error[E0034]: multiple applicable items in scope
    // s.test();
    // 只能类似借助静态方法的调用去区分
    A::test(&s);
    // 推荐用下面这种可读性更高的写法
    <S as B>::test(&s);

    let a = Some("a".to_string());
    if let Some(s) = a {
        println!("{}", s)
    }
    // value borrowed here after partial move
    // println!("{:?}", a);
}
