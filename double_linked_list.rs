use std::cmp::PartialOrd;

pub struct DoubleLinkedList<T: PartialOrd> (Option<(Option<Box<DoubleLinkedList<T>>>, T, Box<DoubleLinkedList<T>>)>);

impl<T: PartialOrd> DoubleLinkedList<T> {
    pub fn new() -> Self {
        DoubleLinkedList(None)
    }

    pub fn push_front(&mut self, data: T) {
        todo!();
    }
}