#[cfg(test)]
mod tests {
    use crate::{linear_search_with_for, linearSearch};

    #[test]
    fn linear_search() {
        let arr: [i32; 5] = [1, 100, 90, 23, 233];
        let target: i32 = 90;

        assert_eq!(linear_search_with_for(&arr, target), 90);
    }
}