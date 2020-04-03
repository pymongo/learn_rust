// mod tests;

fn main() {
  println!("Hello, world!");
  // tests::add
}

mod some_tests {
  #[test]
  fn test_1() {
    assert_eq!(1, 2);
  }
}
