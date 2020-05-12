// u8指的是uint8
fn need_to_test3() -> u8 {
    3
}

#[cfg(test)] // 仅在测试环境下编译
mod tests {
    #[test]
    #[ignore]
    fn test_1() {
        assert_eq!(1, 1);
        assert_ne!(1, 2);
    }

    #[test]
    #[should_panic] // 会pass掉panic抛异常的测试用例
    fn test_2() {
        panic!("throw/raise my exception");
    }

    #[test]
    fn test_3() {
        // super::能获取到module外面的作用域的函数
        assert_eq!(super::need_to_test3(), 3);
    }
}
