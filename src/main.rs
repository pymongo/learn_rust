mod learn;

struct Philosopher {
  name: String,
}

impl Philosopher {
  fn new(name: &str) -> Philosopher {
    Philosopher {
      name: name.to_string(),
    }
  }

  fn eat(&self) {
    println!("{} is eating.", self.name);

    std::thread::sleep(std::time::Duration::from_millis(1000));

    println!("{} is done eating.", self.name);
  }
}

fn main() {
  // module的演示
  learn::read_file::read_file();

  let philosophers = vec![
    Philosopher::new("Baruch Spinoza"),
    Philosopher::new("Gilles Deleuze"),
    Philosopher::new("Karl Marx"),
    Philosopher::new("Friedrich Nietzsche"),
    Philosopher::new("Michel Foucault"),
  ];

  let handles: Vec<_> = philosophers.into_iter().map(|p| {
    // thread::spawn 定义一段在新线程运行的代码块
    std::thread::spawn(move || {
      // [annotation move]: the closure is going to
      // take ownership of the values it’s capturing
      p.eat();
    })
  }).collect();

  for h in handles {
    // 多线程
    h.join().unwrap();
  }
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
