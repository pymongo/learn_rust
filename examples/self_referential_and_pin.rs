/*!
[self-referential types](https://rust-lang.github.io/async-book/04_pinning/01_chapter.html)
*/
use std::pin::Pin;

struct BadSelfReferential {
    data: String,
    ptr: *const String,
}

impl BadSelfReferential {
    fn new(s: &str) -> Self {
        let mut this = BadSelfReferential {
            data: s.to_string(),
            ptr: std::ptr::null(),
        };
        this.ptr = &this.data as *const String;
        this
    }

    fn get_data_by_self_ptr(&self) -> &String {
        unsafe { &*self.ptr }
    }
}

impl std::fmt::Debug for BadSelfReferential {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
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

/// Pin to stack memory
struct GoodSelfReferential {
    data: String,
    ptr: *const String,
    /// This makes our type `!Unpin`
    _exclamation_mark_unpin: std::marker::PhantomPinned
}

impl GoodSelfReferential {
    fn new(s: &str) -> Self {
        let mut this = Self {
            data: s.to_string(),
            ptr: std::ptr::null(),
            _exclamation_mark_unpin: std::marker::PhantomPinned
        };
        // FIXME 错误示例，应让Self被Pin以后再初始化self.ptr，才能指向Pin的内存地址
        this.ptr = &this.data as *const String;
        this
    }

    /// 通常为!Unpin类型创建Pin需要用Box::pin API, Unpin类型可以用Pin::new() API
    fn new_and_pin_to_heap(s: &str) -> Pin<Box<Self>> {
        let this = Self {
            data: s.to_string(),
            ptr: std::ptr::null(),
            _exclamation_mark_unpin: std::marker::PhantomPinned
        };
        // 先装入Pin中再初始化ptr才能保证ptr指向了正确的data
        let mut boxed = Box::pin(this);
        let self_ptr: *const String = &boxed.as_ref().data;
        // Pin之后不能通过safe代码拿到&mut T
        unsafe { boxed.as_mut().get_unchecked_mut().ptr = self_ptr }
        boxed
    }

    fn get_data_by_self_ptr(self: Pin<&mut Self>) -> &String {
        unsafe { &*self.ptr }
    }

    #[cfg(FALSE)]
    fn init(self: Pin<&mut Self>) {
        let self_ptr = &self.data as *const String;
        let this = unsafe { self.get_unchecked_mut() };
        this.ptr = self_ptr;
    }
}

fn main() {
    let mut a = BadSelfReferential::new("a");
    let mut b = BadSelfReferential::new("b");
    dbg!(&a, &b);
    std::mem::swap(&mut a, &mut b);
    /* expected *a.ptr == "b", but got "a"
    a.ptr still points to old location which now is inside b.data after swap
    a = SelfReferential { data: b, *ptr: a }
    b = SelfReferential { data: a, *ptr: b }
    */
    dbg!(&a, &b);

    /*
    within `GoodSelfReferential`, the trait `Unpin` is not implemented for `PhantomPinned`
    let mut c = GoodSelfReferential::new("c");
    let mut d = GoodSelfReferential::new("d");
    // must shadow the original variable, because c/d is not move to Pin
    // but it is dangerous to use c/d after Pin::new_unchecked
    let mut c = unsafe { Pin::new_unchecked(&mut c_raw) };
    let mut d = unsafe { Pin::new_unchecked(&mut d_raw) };
    // 不要再读写原来的c和d，因为已经扔给Pin了，再读原来的c和d就不安全violates the Pin contract
    println!("*c.ptr={}, *d.ptr={}", c.get_data_by_self_ptr(), d.get_data_by_self_ptr());
    std::mem::swap(c.get_mut(), d.get_mut());
    println!("After swap c and d");
    println!("*c.ptr={}, *d.ptr={}", c.get_data_by_self_ptr(), d.get_data_by_self_ptr());
    */
}
