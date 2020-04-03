// mod tests;

fn main() {
  println!("Hello, world!");
  // tests::add
}

#[cfg(test)] // 仅在测试环境下编译
mod some_tests {
  #[test]
  fn test_1() {
    assert_eq!(1, 1);
    assert_ne!(1, 2);
  }

  #[test]
  // #[should_panic] 会pass掉panic抛异常的测试用例
  fn test_2() {
    panic!("throw/raise my exception");
  }
}
