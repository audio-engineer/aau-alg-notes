use std::io::{Error, ErrorKind};

#[derive(Debug, Default)]
pub struct Stack {
    array: Vec<i32>,
    top: usize,
}

impl Stack {
    pub fn new(size: usize) -> Self {
        Stack {
            array: vec![0; size],
            top: 0,
        }
    }

    pub fn push(&mut self, value: i32) -> Result<(), Error> {
        if self.top == self.array.capacity() {
            return Err(Error::new(ErrorKind::Other, "Overflow".to_string()));
        }

        self.array[self.top] = value;

        self.top += 1;

        Ok(())
    }

    pub fn pop(&mut self) -> Result<i32, Error> {
        if self.top == 0 {
            return Err(Error::new(ErrorKind::Other, "Underflow".to_string()));
        }

        self.top -= 1;

        Ok(self.array[self.top])
    }
}
