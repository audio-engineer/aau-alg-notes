pub use self::heap_sort::*;
pub use self::insertion_sort::*;
pub use self::merge_sort::*;
pub use self::quick_sort::*;
pub use self::reverse::*;

mod heap_sort;
mod insertion_sort;
mod merge_sort;
mod quick_sort;
mod reverse;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reverse_array() {
        let mut array = [5, 4, 6, 8, 20, 56, 7, 9, 31];

        reverse(&mut array);

        assert_eq!(array, [31, 9, 7, 56, 20, 8, 6, 4, 5]);
    }

    #[test]
    fn sort_with_insertion_sort() {
        let mut array = [5, 4, 6, 8, 20, 56, 7, 9, 31];

        insertion_sort(&mut array);

        assert_eq!(array, [4, 5, 6, 7, 8, 9, 20, 31, 56]);
    }

    #[ignore]
    #[test]
    fn sort_with_recursive_insertion_sort() {
        let mut array = [5, 4, 6, 8, 20, 56, 7, 9, 31];
        let array_length = array.len();

        recursive_insertion_sort(&mut array, array_length);

        assert_eq!(array, [4, 5, 6, 7, 8, 9, 20, 31, 56]);
    }

    #[test]
    fn sort_with_merge_sort() {
        let mut array = [12, 3, 7, 9, 14, 6, 11, 2];
        let array_length = array.len();

        merge_sort(&mut array, 0, array_length);

        assert_eq!(array, [2, 3, 6, 7, 9, 11, 12, 14]);
    }

    #[test]
    fn sort_with_merge_sort_two() {
        let mut array = [3, 41, 52, 26, 38, 57, 49, 9];
        let array_length = array.len();

        merge_sort(&mut array, 0, array_length);

        assert_eq!(array, [3, 9, 26, 38, 41, 49, 52, 57]);
    }

    #[test]
    fn sort_with_heap_sort() {
        let mut array = [3, 41, 52, 26, 38, 57, 49, 9];

        heap_sort(&mut array);

        assert_eq!(array, [3, 9, 26, 38, 41, 49, 52, 57]);
    }

    #[test]
    fn sort_with_quick_sort() {
        let mut array = [3, 41, 52, 26, 38, 57, 49, 9];
        let array_length = array.len();

        quick_sort(&mut array, 0, array_length - 1);

        assert_eq!(array, [3, 9, 26, 38, 41, 49, 52, 57]);
    }
}
