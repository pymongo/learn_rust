// 此时state相当于全局变量，比mut static更好的是不用unsafe语句了
#[allow(dead_code)]
fn outer1() -> impl FnMut() -> bool {
  let mut state = false;
  move || {
    state = !state;
    return state;
  }
}

// fn outer2() -> impl FnMut() {
//   let mut state = false;
//   move || {
//     state = !state;
//     println!("state = {}", state);
//   }
// }

// static mut GLOBAL_STATE: bool = false;
fn main() {
  // 必须在unsafe块里才能修改全局static变量
  // unsafe {
  //   GLOBAL_STATE = true;
  // }
  let mut f = outer1();
  dbg!(f()); // true
  dbg!(f()); // false
  dbg!(f()); // true
  // outer2()();
}

/* python版写法，需要nonlocal关键词
def outer():
    state = False
    def inner():
        nonlocal state
        state = not state
        print(f"state = {state}")
    return inner

func = outer()
func()
func()
func()
*/