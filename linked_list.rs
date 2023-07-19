use std::cmp::PartialOrd;

// Tuple struct
#[derive(Debug)]
pub struct LinkedList<T: PartialOrd> (Option<(T, Box<LinkedList<T>>)>);

impl<T: PartialOrd> LinkedList<T> {
    pub fn new() -> Self {
        return LinkedList(None);
    }

    pub fn push_front(&mut self, data: T) {
        let t = self.0.take();
        self.0 = Some((data, Box::new(LinkedList(t))));
    }

    pub fn push_back(&mut self, data: T) {
        match self.0 {
            Some((_, ref mut child)) => child.push_back(data),
            None => self.push_front(data)
        }
    }

    pub fn push(&mut self, data: T, index: u32) {
        if index == 0 {
            self.push_front(data);
        }
        else {
            match self.0 {
                Some((_, ref mut child)) => child.push(data, index - 1),
                None => self.push_front(data)
            }
        }
    }
}

fn main() {
    let mut ll = LinkedList::new();
    ll.push_back(4);
    ll.push_front(3);
    ll.push_back(8);
    ll.push(2, 2);

    println!("{:?}", ll);
}