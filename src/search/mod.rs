pub use self::minimum::*;

mod minimum;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn two_smallest_found() {
        let mut array = [5, 4, 6, 8, 20, 56, 7, 9, 31];

        let result = find_two_smallest(&mut array);

        assert_eq!(result, (4, 5));
    }

    #[test]
    fn find_minimum_with_incremental() {
        let mut array = [5, 4, 6, 8, 20, 56, 7, 9, 31];

        let minimum = find_minimum(&mut array);

        assert_eq!(minimum, 4);
    }

    #[ignore]
    #[test]
    fn find_minimum_with_recursive() {
        let mut array = [5, 4, 6, 8, 20, 56, 7, 9, 31];
        let array_length = array.len();

        let minimum = find_minimum_recursive(&mut array, 0, array_length);

        assert_eq!(minimum, 4);
    }
}
