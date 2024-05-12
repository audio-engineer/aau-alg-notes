pub fn reverse(array: &mut [i32]) {
    let array_length = array.len();

    for i in 0..array_length / 2 {
        array.swap(i, array_length - 1 - i);
    }
}
