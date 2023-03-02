#[cfg(test)]
mod tests {
    use crate::{addition, fizz_buzz};
    use crate::subtraction;

    #[test]
    fn test_addition() {
        assert_eq!(addition(7i32, 7i32), 14i32);
    }

    #[test]
    fn test_subtraction() {
        assert_eq!(subtraction(15i32, 10i32), 5i32);
    }

    #[test]
    fn test_fizz_buzz() {
        assert_eq!(fizz_buzz(15), "FizzBuzz")
    }

    #[test]
    fn test_fizz() {
        assert_eq!(fizz_buzz(3), "Fizz")
    }

    #[test]
    fn test_buzz() {
        assert_eq!(fizz_buzz(5), "Buzz")
    }

    #[test]
    fn test_number_invalid() {
        assert_eq!(fizz_buzz(1), "number invalid")
    }
}