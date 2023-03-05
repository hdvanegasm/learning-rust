fn prints_and_returns_10(a: i32) -> i32 {
    println!("I got the value {}", a);
    10
}

#[cfg(test)]
mod tests {
    use super::*;

    // cargo test -- --show-output : to show the output for methods that print
    // something in stdout.
    #[test]
    fn this_test_will_pass() {
        let value = prints_and_returns_10(4);
        assert_eq!(10, value);
    }

    #[test]
    #[ignore]
    fn this_test_will_fail() {
        let value = prints_and_returns_10(4);
        assert_eq!(5, value);
    }

    #[test]
    fn test1() {
        let value = prints_and_returns_10(3);
        assert_ne!(value + 1, 10);
    }
    
    #[test]
    fn test2() {
        let value = prints_and_returns_10(3);
        assert_ne!(value + 2, 10);
    }

    #[test]
    fn test3() {
        let value = prints_and_returns_10(3);
        assert_ne!(value + 3, 10);
    }
}
