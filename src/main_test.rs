#[cfg(test)]
mod tests {
    use crate::addition;
    use crate::subtraction;

    #[test]
    fn test_addition() {
        assert_eq!(addition(7i32, 7i32), 14i32);
    }

    #[test]
    fn test_subtraction() {
        assert_eq!(subtraction(15i32, 10i32), 5i32);
    }
}