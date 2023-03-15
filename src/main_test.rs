#[cfg(test)]
mod tests {
    use crate::{linear_search_for_i16, linear_search_for_i32};

    #[test]
    fn linear_search_i32_when_element_is_found() {
        let arr: [i32; 5] = [1, 100, 90, 23, 233];
        let target: i32 = 90;

        assert_eq!(linear_search_for_i32(&arr, target), 90);
    }

    #[test]
    fn linear_search_i32_when_element_not_found() {
        let arr: [i32; 5] = [1, 100, 90, 23, 233];
        let target: i32 = 101;

        assert_eq!(linear_search_for_i32(&arr, target), -1);
    }

    #[test]
    fn linear_search_i16_when_element_is_found() {
        let arr: [i16; 5] = [1, 100, 90, 23, 233];
        let target: i16 = 90;

        assert_eq!(linear_search_for_i16(&arr, target), 90);
    }

    #[test]
    fn linear_search_i16_when_element_not_found() {
        let arr: [i16; 5] = [1, 100, 90, 23, 233];
        let target: i16 = 101;

        assert_eq!(linear_search_for_i16(&arr, target), -1);
    }
}