pub trait LinkedNode {
    fn new(key: i32) -> Self;
}

#[derive(Default)]
enum Link<T: LinkedNode> {
    #[default]
    Empty,
    More(Box<T>),
}

struct SinglyLinkedNode {
    key: i32,
    next: Link<SinglyLinkedNode>,
}

impl LinkedNode for SinglyLinkedNode {
    fn new(key: i32) -> Self {
        SinglyLinkedNode {
            key,
            next: Link::Empty,
        }
    }
}

struct DoublyLinkedNode {
    key: i32,
    previous: Link<DoublyLinkedNode>,
    next: Link<DoublyLinkedNode>,
}

impl LinkedNode for DoublyLinkedNode {
    fn new(key: i32) -> Self {
        DoublyLinkedNode {
            key,
            previous: Link::Empty,
            next: Link::Empty,
        }
    }
}

#[derive(Default)]
pub struct LinkedList<T: LinkedNode> {
    head: Link<T>,
}

impl<T: LinkedNode> LinkedList<T> {
    pub fn new() -> Self {
        LinkedList { head: Link::Empty }
    }

    pub fn prepend(&mut self, key: i32)
    where
        T: LinkedNode,
    {
        self.head = Link::More(Box::new(T::new(key)));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn singly_prepend_underflow() {
        let mut singly_linked_list = LinkedList::<SinglyLinkedNode>::new();
        singly_linked_list.prepend(2);

        // assert_eq!(latest_value, 3);
    }
}
