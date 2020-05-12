use std::sync::{Arc, Mutex};
use std::thread;

struct Philosopher {
    name: String,
    left: usize,
    // usize, uint_32
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
        // 使用_left和_right的目的是为了让eat方法结束时，rust释放变量内存时把线程锁解开
        let _left = table.forks[self.left].lock().unwrap();
        let _right = table.forks[self.right].lock().unwrap();
        println!(
            "fork {} and {} is using by {}",
            self.left, self.right, self.name
        );

        println!("{} is eating.", self.name);

        thread::sleep(std::time::Duration::from_millis(1000));

        println!("{} is done eating.", self.name);
    }
}

// 餐桌
struct Table {
    // Mutex类似Java的synchronized，线程锁
    // We use an empty tuple, (), inside the mutex, since we’re not actually going to use the value, just hold onto it.
    forks: Vec<Mutex<()>>,
}

/// ### Rust Arc<Mutex<T>> pattern
/// An extremely common pattern in Rust is Arc<Mutex<T>>,
/// where Arc provides the memory management,
/// and Mutex provides safe multi-threaded access to the resource
fn main() {
    // [reference count]引用计数是计算机编程语言中的一种内存管理技术，是指将资源（可以是对象、内存或磁盘空间等等）的被引用次数保存起来，
    // 当被引用次数变为零时就将其释放的过程。使用引用计数技术可以实现自动资源管理的目的。同时引用计数还可以指使用引用计数技术回收未使用资源的垃圾回收算法。
    // ARC = atomic reference count， 用于多线程间共享变量
    // share our Table across multiple threads
    let table = Arc::new(Table {
        forks: vec![
            Mutex::new(()),
            Mutex::new(()),
            Mutex::new(()),
            Mutex::new(()),
            Mutex::new(()),
        ],
    });

    let philosophers = vec![
        Philosopher::new("Philosopher_1", 0, 1),
        Philosopher::new("Philosopher_2", 1, 2),
        Philosopher::new("Philosopher_3", 2, 3),
        Philosopher::new("Philosopher_4", 3, 4),
        Philosopher::new("Philosopher_5", 0, 4),
    ];

    let handlers: Vec<_> = philosophers
        .into_iter()
        .map(|p| {
            // Arc<T> is what bumps up the reference count, and when it goes out of scope, it decrements the count
            // 简单来说通过克隆，复制一份餐桌的5个叉子的应用，当线程结束时，把这5个新的叉子引用释放掉
            // You’ll notice we can introduce a new binding to table here, and it will shadow the old one.
            let table = table.clone();
            thread::spawn(move || {
                p.eat(&table);
            })
        })
        .collect();

    for handler in handlers {
        // 多线程
        handler.join().unwrap();
    }
}
