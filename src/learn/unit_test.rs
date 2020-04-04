#[cfg(test)] // 仅在测试环境下编译
pub mod tests {
  #[test]
  fn test_1() {
    assert_eq!(1, 1);
    assert_ne!(1, 2);
  }

  #[test]
  // #[should_panic] // 会pass掉panic抛异常的测试用例
  #[ignore]
  fn test_2() {
    panic!("throw/raise my exception");
  }

  #[test]
  fn test_3() {
    // super::能获取到module外面的作用域的函数
    // assert_eq!(super::need_to_test3(), 3);
    assert_eq!(need_to_test3(), 3);
  }

  // u8指的是uint8
  // TODO 其实忽视这个警告的最佳实践是将这个函数设为「仅在测试环境下编译」，毕竟只在测试环境下使用
  #[allow(dead_code)] // 忽略未使用的代码的警告
  fn need_to_test3() -> u8 {
    3
  }
}