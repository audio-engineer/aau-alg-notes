fn partition(array: &mut [i32], low: usize, high: usize) -> usize {
    let pivot = high;
    let mut i = low;

    for j in low..high {
        if array[j] < array[pivot] {
            array.swap(j, i);

            i += 1;
        }
    }

    array.swap(pivot, i);

    i
}

pub fn quick_sort(array: &mut [i32], low: usize, high: usize) {
    if low < high {
        let partition_index = partition(array, low, high);

        quick_sort(array, low, partition_index - 1);
        quick_sort(array, partition_index + 1, high);
    }
}
