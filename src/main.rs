extern crate rand;
use rand::Rng;

fn main() {
  let tuple = (0,1.1,(false, "1"));
  println!("{}", (tuple.2).1);
  let (a, b) = tuple.2;
  println!("a={}, b={}", a, b);
  let random_number : u8 = rand::thread_rng().gen_range(1, 101);
  println!("{}", random_number);
  match 50.cmp(&random_number) {
    std::cmp::Ordering::Less    => println!("Too small!"),
    std::cmp::Ordering::Greater => println!("Too big!"),
    std::cmp::Ordering::Equal   => println!("You win!"),
  }
  // thread_rng() function to get a copy of the random number generator, which is local to the particular thread of execution we’re in
  // Because we use rand::Rng’d above, it has a gen_range() method available
}

// u8指的是uint8
// TODO 其实忽视这个警告的最佳实践是将这个函数设为「仅在测试环境下编译」，毕竟只在测试环境下使用
#[allow(dead_code)] // 忽略未使用的代码的警告
fn need_to_test3() -> u8 {
  3
}

#[cfg(test)] // 仅在测试环境下编译
mod some_tests {
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
    assert_eq!(super::need_to_test3(), 3);
  }
}
