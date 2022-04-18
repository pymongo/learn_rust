#![feature(scoped_threads)]
use std::sync::{
    atomic::{AtomicU8, Ordering},
    Arc, Mutex,
};

static PHILOSOPHER_INDEX: AtomicU8 = AtomicU8::new(0);

struct Philosopher {
    name: String,
    left: usize,
    right: usize, // left, right指的是哲学家的左右手在餐桌的几号叉子上
}

impl Philosopher {
    fn new(left: usize, right: usize) -> Philosopher {
        let index = PHILOSOPHER_INDEX.fetch_add(1, Ordering::SeqCst);
        Philosopher {
            name: format!("philosopher_{}", index),
            left,
            right,
        }
    }

    fn eat(&self, table: &Table) {
        // 使用_left和_right的目的是为了让eat方法结束时，rust释放变量内存时把线程锁解开
        let _left = table.forks[self.left].lock().unwrap();
        let _right = table.forks[self.right].lock().unwrap();
        println!("{} is eating.", self.name);
        println!(
            "\tfork {} and {} is using by {}",
            self.left, self.right, self.name
        );

        std::thread::sleep(std::time::Duration::from_millis(500));
        println!("{} is done eating.", self.name);
    }
}

struct Table {
    // We use an empty tuple, (), inside the mutex, since we’re not actually going to use the value, just hold onto it.
    forks: Vec<Mutex<()>>,
}

fn init_table_and_philosophers() -> (Table, Vec<Philosopher>) {
    (
        Table {
            forks: vec![
                Mutex::new(()),
                Mutex::new(()),
                Mutex::new(()),
                Mutex::new(()),
                Mutex::new(()),
            ],
        },
        vec![
            Philosopher::new(0, 1),
            Philosopher::new(1, 2),
            Philosopher::new(2, 3),
            Philosopher::new(3, 4),
            Philosopher::new(0, 4),
        ],
    )
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
    let (table, philosophers) = init_table_and_philosophers();
    let table = Arc::new(table);

    // Arc<T> is what bumps up the reference count, and when it goes out of scope, it decrements the count
    // 简单来说通过克隆，复制一份餐桌的5个叉子的应用，当线程结束时，把这5个新的叉子引用释放掉
    // You’ll notice we can introduce a new binding to table here, and it will shadow the old one.
    let handles: Vec<_> = philosophers
        .into_iter()
        .map(|philosopher| {
            let table = table.clone();
            std::thread::spawn(move || {
                philosopher.eat(&table);
            })
            // 不能这里 join 否则会 one-by-one 逐个运行达不到并行执行效果
        })
        .collect();
    handles
        .into_iter()
        .for_each(|handle| handle.join().unwrap());
}

#[test]
fn solve_by_scoped_thread() {
    let (table, philosophers) = init_table_and_philosophers();
    std::thread::scope(|scope| {
        philosophers.into_iter().for_each(|philosopher| {
            let table = &table;
            scope.spawn(move || {
                philosopher.eat(table);
            });
        });
    });
}
