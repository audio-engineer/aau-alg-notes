fn merge(array: &mut [i32], index_left: usize, index_middle: usize, index_right: usize) {
    let length_left = index_middle - index_left;
    let length_right = index_right - index_middle;

    // Copy elements from the original array into two subarrays
    let mut array_left = vec![0; length_left];
    array_left.copy_from_slice(&array[index_left..index_middle]);

    let mut array_right = vec![0; length_right];
    array_right.copy_from_slice(&array[index_middle..index_right]);

    let mut i = 0;
    let mut j = 0;
    let mut k = index_left;

    while i < length_left && j < length_right {
        if array_left[i] <= array_right[j] {
            array[k] = array_left[i];
            i += 1;
        } else {
            array[k] = array_right[j];
            j += 1;
        }

        k += 1;
    }

    while i < length_left {
        array[k] = array_left[i];
        i += 1;
        k += 1;
    }

    while j < length_right {
        array[k] = array_right[j];
        j += 1;
        k += 1;
    }
}

pub fn merge_sort(array: &mut [i32], index_left: usize, index_right: usize) {
    if index_right - 1 > index_left {
        let index_middle = index_left + (index_right - index_left) / 2;

        merge_sort(array, index_left, index_middle);
        merge_sort(array, index_middle, index_right);

        merge(array, index_left, index_middle, index_right);
    }
}
