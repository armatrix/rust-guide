#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
    #[test]
    fn foo() {
        panic!("failed")
    }
    use super::*;

    #[test]
    #[should_panic(expected = "some message")]
    fn gt100() {
        Guess::new(233);
    }

    #[test]
    fn bar() -> Result<(), String> {
        if 2 + 2 == 4 {
            return Ok(());
        }
        Err(String::from("not expected"))
    }
}

pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 || value > 100 {
            panic!("not in 1..100")
        }
        Guess { value }
    }
}
