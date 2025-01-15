pub use self::hash_table::*;
pub use self::linked_list::*;
pub use self::priority_queue::*;
pub use self::queue::*;
pub use self::stack::*;

mod hash_table;
mod linked_list;
mod priority_queue;
mod queue;
mod stack;

#[cfg(test)]
mod tests {
    use std::io::ErrorKind;

    use super::*;

    #[test]
    fn stack_pop() {
        let mut stack = Stack::new(5);

        stack.push(3).expect("Push error");
        let latest_value = stack.pop().expect("Pop error");

        assert_eq!(latest_value, 3);
    }

    #[test]
    fn stack_overflow() {
        let mut stack = Stack::new(2);

        stack.push(3).expect("Push error");
        stack.push(3).expect("Push error");

        assert_eq!(ErrorKind::Other, stack.push(3).unwrap_err().kind());
    }

    #[test]
    fn stack_underflow() {
        let mut stack = Stack::new(2);

        assert_eq!(ErrorKind::Other, stack.pop().unwrap_err().kind());
    }

    #[ignore]
    #[test]
    fn queue_overflow() {
        let mut queue = Queue::new(2);
        queue.enqueue(2).expect("");
        queue.enqueue(2).expect("");

        assert_eq!(ErrorKind::Other, queue.enqueue(2).unwrap_err().kind());
    }

    #[test]
    fn queue_underflow() {
        let mut queue = Queue::new(1);

        assert_eq!(ErrorKind::Other, queue.dequeue().unwrap_err().kind());
    }
}
