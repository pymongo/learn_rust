// 此时state相当于全局变量，比mut static更好的是不用unsafe语句了
fn outer1() -> impl FnMut() -> bool {
    let mut state = false;
    move || {
        state = !state;
        state
    }
}

fn main() {
    let mut f = outer1();
    dbg!(f()); // true
    dbg!(f()); // false
    dbg!(f()); // true
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
