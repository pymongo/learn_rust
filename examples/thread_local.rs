use std::cell::RefCell;
use std::borrow::Borrow;

static MARKET_ID: RefCell<String> = RefCell::new(String::new());

// 第二版内容: Once和PhantomData
fn main() {
    // let _ = std::panic::catch_unwind(|| {panic!("Something Wrong")});
    // 为了让static变量具有可变性，建议用Cell或RefCell，由于这里不希望u32被Copy，所以用RefCell提供指针去修改u23
    thread_local! {static DATA: RefCell<u32> = RefCell::new(1)};
    std::thread::spawn(|| {
       DATA.with(|f| {
           assert_eq!(*f.borrow(), 1);
           // 在另一个线程中将DATA改为2
           *f.borrow_mut() = 2;
       });
    });
    // DATA的类型: std::thread::LocalKey<std::cell::RefCell<u32>>
    DATA.with(|f| {
        // 在main线程中DATA还是1
        assert_eq!(*f.borrow(), 1);
    });
}