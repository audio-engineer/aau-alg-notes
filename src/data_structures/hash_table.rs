#[derive(Debug, Default)]
pub struct HashTable {
    array: Vec<i32>,
}

impl HashTable {
    pub fn new(size: usize) -> Self {
        HashTable {
            array: vec![0; size],
        }
    }

    fn hash(&mut self, value: i32) -> usize {
        value as usize % self.array.capacity()
    }

    pub fn insert(&mut self, value: i32) {
        let hash = self.hash(value);

        self.array[hash] = value;
    }

    pub fn delete(&mut self, value: i32) {
        let hash = self.hash(value);

        self.array[hash] = 0;
    }
}
