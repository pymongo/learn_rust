fn main() {
    let mut iter = vec![1,2,3,4,5].into_iter();
    assert_eq!(Some(1), iter.next());
    assert_eq!(Some(5), iter.next_back());
    assert_eq!(Some(4), iter.next_back());
    // 所以向前和向后遍历的游标似乎是两个独立的指针，互不影响
    assert_eq!(Some(2), iter.next());
}