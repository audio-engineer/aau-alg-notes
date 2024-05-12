use crate::sorting::heapify;

pub fn insert(array: &mut [i32]) {}

pub fn maximum(array: &mut [i32]) -> i32 {
    array[0]
}

pub fn extract_max(array: &mut [i32], heap_size: usize) -> i32 {
    let maximum = maximum(array);

    array[0] = array[heap_size - 1];

    heapify(array, heap_size - 1, 1);

    maximum
}

pub fn increase_key(array: &mut [i32]) {}
