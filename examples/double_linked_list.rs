use std::cell::RefCell;
use std::rc::{Rc, Weak};

struct Node<T> {
    data: T,
    prev: Option<Weak<RefCell<Node<T>>>>,
    next: Option<Rc<RefCell<Node<T>>>>,
}

impl<T> Node<T> {
    fn new(val: T) -> Self {
        Node {
            data: val,
            prev: None,
            next: None,
        }
    }
}

struct List<T> {
    // dummy head and tail
    first: Node<T>,
    tail: Node<T>,
}

// impl List<T> {
//     fn new(dummy_val: T) -> Self {
//         let mut head = Node::new(dummy_val);
//         let mut tail = Node::new(dummy_val);
//         head.next = Some(Rc::new(tail));
//         tail.prev = Some(Weak::new(head));
//         List {
//             first: Node::new(-1),
//             tail: Node::new(-1)
//         }
//     }
// }

fn main() {
    let mut head: Node<i32> = Node::new(-1);
}
