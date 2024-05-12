/// Finds the two smallest numbers in an array of numbers and returns them.
pub fn find_two_smallest(array: &mut [i32]) -> (i32, i32) {
    let mut smallest_element_one = array[0];
    let mut smallest_element_two = array[0];

    for current_element in array {
        if current_element < &mut smallest_element_one {
            smallest_element_two = smallest_element_one;

            smallest_element_one = *current_element;
        }

        if current_element < &mut smallest_element_two
            && current_element != &mut smallest_element_one
        {
            smallest_element_two = *current_element;
        }
    }

    (smallest_element_one, smallest_element_two)
}

/// Finds the smallest number in an array of numbers and returns it.
pub fn find_minimum(array: &mut [i32]) -> i32 {
    let mut minimum = array[0];

    for i in array {
        if i < &mut minimum {
            minimum = *i;
        }
    }

    minimum
}

/// Finds the smallest number in an array of numbers using a recursive algorithm and returns it.
pub fn find_minimum_recursive(array: &mut [i32], p: usize, q: usize) -> i32 {
    if p == q {
        array[p]
    } else {
        let middle_index = p + q / 2;

        let min_l = find_minimum_recursive(array, p, middle_index);
        let min_r = find_minimum_recursive(array, middle_index + 1, q);

        if min_l < min_r {
            min_l
        } else {
            min_r
        }
    }
}
