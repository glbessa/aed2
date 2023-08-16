

pub struct Tree<T> {
    root: Option<Box<Node<T>>>
}

pub struct Node<T> {
    value: T,
    left: Option<Box<Node<T>>>,
    right: Option<Box<Node<T>>>
}

impl<T> Tree<T> {
    pub fn new() -> Self {
        Tree {root: None}
    }
}

impl<T> Node<T> {
    pub fn new(v: T) -> Self {
        Node {value: v, left: None, right: None}
    }
}