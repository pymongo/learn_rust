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