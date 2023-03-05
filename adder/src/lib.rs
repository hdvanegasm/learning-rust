#[derive(Debug)]
struct Rectangle {
    width: u32,
    heigth: u32,
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.heigth > other.heigth
    }
}

fn add_two(a: i32) -> i32 {
    a + 2
}

fn greeting(name: &str) -> String {
    format!("Hello")
}

struct Guess {
    value: i32,
}

impl Guess {
    fn new(value: i32) -> Self {
        if value < 1 {
            panic!("Guess must be greater than 1. Got {}", value);
        } else if value > 100 {
            panic!("Guest must be less than or equal to 100. Got {}", value)
        }

        Guess { value }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn larger_can_hold_smaller() {
        let larger = Rectangle {
            width: 8,
            heigth: 7,
        };

        let smaller = Rectangle {
            width: 5,
            heigth: 1,
        };

        assert!(larger.can_hold(&smaller));
    }

    #[test]
    fn add_two_correctly() {
        assert_eq!(4, add_two(2))
    }

    #[test]
    fn smaller_cannot_hold_larger() {
        let larger = Rectangle {
            width: 8,
            heigth: 7,
        };

        let smaller = Rectangle {
            width: 5,
            heigth: 1,
        };

        assert!(!smaller.can_hold(&larger))
    }

    #[test]
    fn greeting_contains_name() {
        let result = greeting("Carol");

        // The optional arguments are passed through a format! macro before and
        // then they are printed in case of failure.
        assert!(
            result.contains("Carol"),
            "Greeting did not contain name, value was {}",
            result
        )
    }

    // To make the should_panic more precise, we add the expected attribute.
    // This attribute specifies that the panic message should contain the value
    // of the expected attribute as a sub-string. In this way, we avoid that the
    // test passes because another random reason that panics too.
    #[test]
    #[should_panic(expected = "less than or equal to 100")]
    fn greater_than_100() {
        Guess::new(200);
    }
}
