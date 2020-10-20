/*!
[self-referential types](https://rust-lang.github.io/async-book/04_pinning/01_chapter.html)
*/

use serde::export::Formatter;

struct BadSelfReferential {
    data: String,
    ptr: *const String,
}

impl BadSelfReferential {
    fn new(s: &str) -> Self {
        let mut a = BadSelfReferential {
            data: s.to_string(),
            ptr: std::ptr::null(),
        };
        a.ptr = &a.data as *const String;
        a
    }

    fn get_data_by_self_ptr(&self) -> &String {
        unsafe { &*self.ptr }
    }
}

impl std::fmt::Debug for BadSelfReferential {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "BadSelfReferential {{ data: {}, *ptr: {} }}",
            self.data,
            self.get_data_by_self_ptr()
        )
        // f.debug_struct("SelfReferential")
        //     .field("data", &self.data)
        //     .field("*ptr", self.get_data_by_self_ptr())
        //     .finish()
    }
}

fn main() {
    let mut a = BadSelfReferential::new("a");
    let mut b = BadSelfReferential::new("b");
    dbg!(&a);
    dbg!(&b);
    std::mem::swap(&mut a, &mut b);
    /* expected *a.ptr == "b", but got "a"
    a = SelfReferential { data: b, *ptr: a }
    b = SelfReferential { data: a, *ptr: b }
    */
    dbg!(&a);
    dbg!(&b);
}
