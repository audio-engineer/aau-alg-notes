pub fn heapify(array: &mut [i32], heap_size: usize, i: usize) {
    let mut largest = i;

    let left = 2 * i + 1;
    let right = 2 * i + 2;

    if left < heap_size && array[left] > array[largest] {
        largest = left;
    }

    if right < heap_size && array[right] > array[largest] {
        largest = right;
    }

    if largest != i {
        array.swap(i, largest);

        heapify(array, heap_size, largest);
    }
}

fn build_max_heap(array: &mut [i32], heap_size: usize) {
    for i in (0..=heap_size / 2 - 1).rev() {
        heapify(array, heap_size, i);
    }
}

pub fn heap_sort(array: &mut [i32]) {
    let heap_size = array.len();

    build_max_heap(array, heap_size);

    for i in (0..=heap_size - 1).rev() {
        array.swap(0, i);

        heapify(array, i, 0);
    }
}
