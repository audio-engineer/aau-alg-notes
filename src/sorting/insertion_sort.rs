pub fn insertion_sort(array: &mut [i32]) {
    for i in 1..array.len() {
        let key = array[i];

        let mut j = i;

        while j > 0 && array[j - 1] > key {
            array[j] = array[j - 1];

            j -= 1;
        }

        array[j] = key;
    }
}

pub fn recursive_insertion_sort(array: &mut [i32], p: usize) {
    if p <= 1 {
        return;
    }

    recursive_insertion_sort(array, p - 1);

    let key = array[p - 1];
    let mut i = p - 2;

    while array[i] > key {
        array[i + 1] = array[i];

        i -= 1;
    }

    array[i + 1] = key;
}
