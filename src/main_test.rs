#[cfg(test)]
mod tests {
    use crate::addition;

    #[test]
    fn test_addition() {
        assert_eq!(addition(7i32, 7i32), 14i32);
    }
}