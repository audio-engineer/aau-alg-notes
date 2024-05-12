use std::io::{Error, ErrorKind};

#[derive(Debug, Default)]
pub struct Queue {
    array: Vec<i32>,
    tail: usize,
    head: usize,
}

impl Queue {
    pub fn new(size: usize) -> Self {
        Queue {
            tail: 0,
            head: 0,
            array: vec![0; size],
        }
    }

    pub fn enqueue(&mut self, value: i32) -> Result<(), Error> {
        if self.head == self.array.capacity() {
            return Err(Error::new(ErrorKind::Other, "Overflow".to_string()));
        }

        self.array[self.tail] = value;

        self.array[self.tail - 1] = value;

        Ok(())
    }

    pub fn dequeue(&mut self) -> Result<i32, Error> {
        if self.tail == self.head {
            return Err(Error::new(ErrorKind::Other, "Underflow".to_string()));
        }

        if self.head == self.array.capacity() {
            self.head = 0;
        }

        self.head += 1;

        Ok(self.array[self.head])
    }
}
