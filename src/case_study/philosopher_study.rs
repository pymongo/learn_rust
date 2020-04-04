use std::thread;
use std::sync::{Mutex, Arc};

struct Philosopher {
  name: String,
  left: usize, // usize, uint_32
  right: usize, // left, right指的是哲学家的左右手在餐桌的几号叉子上
}

impl Philosopher {
  fn new(name: &str, left: usize, right: usize) -> Philosopher {
    Philosopher {
      name: name.to_string(),
      left,
      right,
    }
  }

  fn eat(&self, table: &Table) {
    let _left = table.forks[self.left].lock().unwrap();
    let _right = table.forks[self.right].lock().unwrap();

    println!("{} is eating.", self.name);

    std::thread::sleep(std::time::Duration::from_millis(1000));

    println!("{} is done eating.", self.name);
  }
}

// 餐桌
struct Table {
  forks: Vec<Mutex<()>>,
}

pub fn run() {
  let table = Arc::new(Table { forks: vec![
    Mutex::new(()),
    Mutex::new(()),
    Mutex::new(()),
    Mutex::new(()),
    Mutex::new(()),
  ]});

  let philosophers = vec![
    Philosopher::new("Philosopher_1", 0, 1),
    Philosopher::new("Philosopher_2", 1, 2),
    Philosopher::new("Philosopher_3", 2, 3),
    Philosopher::new("Philosopher_4", 3, 4),
    Philosopher::new("Philosopher_5", 0, 4),
  ];

  let handles: Vec<_> = philosophers.into_iter().map(|p| {
    // thread::spawn 定义一段在新线程运行的代码块
    std::thread::spawn(move || {
      // [annotation move]: the closure is going to
      // take ownership of the values it’s capturing
      p.eat(&table.clone());
    })
  }).collect();

  for h in handles {
    // 多线程
    h.join().unwrap();
  }
}