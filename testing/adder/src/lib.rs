pub fn add_two(a: i32) -> i32 {
    println!("Execute add_two function.");
    a + 2
}

pub struct Guess {
    value: u32,
}

impl Guess {
    pub fn new(value: u32) -> Guess {
        if value < 1 {
            panic!("Guess value must be greater than or equal to 1, got {}", value);
        } else if value > 100 {
            panic!("Guess value must be less than or equal to 100, got {}", value)
        }

        Guess {
            value
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::add_two;
    use crate::Guess;

    #[test]
    fn exploration() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    #[ignore]
    fn another() {
        panic!("Make this test fail");
    }

    #[test]
    #[ignore]
    fn it_adds_two() {
        assert_eq!(4, add_two(2), "결과값이 같지 않음");
    }

    #[test]
    #[should_panic(expected = "Guess value must be less than or equal to 100")]
    fn greater_than_100() {
        Guess::new(200);
    }
}
