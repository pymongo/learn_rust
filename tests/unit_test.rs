const fn need_to_test3() -> u8 {
    3
}

#[cfg(test)]
mod tests {
    #[test]
    #[ignore]
    fn test_1() {
        assert_ne!(1, 2);
    }

    #[test]
    #[should_panic]
    fn test_2() {
        panic!("should_panic");
    }

    #[test]
    fn test_3() {
        // super::能获取到module test外面的作用域(也就是crate root)
        assert_eq!(super::need_to_test3(), 3);
    }
}
