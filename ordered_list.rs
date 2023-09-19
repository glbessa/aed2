use std::cmp::{Ord};
use std::iter::{Iterator};
use std::ops::{Index, IndexMut};
use std::marker::{Copy};
use std::error::{Error};
use std::fmt;
use std::fmt::{Display, Formatter};

#[derive(Debug)]
struct OutOfSpaceError {
    details: String
}

impl OutOfSpaceError {
    fn new(msg: &str) -> Self {
        OutOfSpaceError {
            details: msg.to_string()
        }
    }
}

impl Display for OutOfSpaceError {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "OutOfSpaceError: {}", self.details)
    }
}

impl Error for OutOfSpaceError {
    fn description(&self) -> &str {
        &self.details
    }
}

pub struct OrderedList<T: Ord + Copy> {
    head: Option<LinkedList<T>>,
    length: usize
}

impl<T: Ord + Copy> OrderedList<T> {
    pub fn new(length: usize) -> Self {
        if length == 0 {
            return OrderedList {
                head: None,
                length: length
            };
        }

        OrderedList {
            head: Some(LinkedList::new()),
            length: length
        }
    }

    pub fn len(&self) -> usize {
        self.length
    }

    pub fn push(&mut self, item: T) -> Result<(), Box<dyn Error>> {
        if self.len() == 0 {
            return Err(Box::new(OutOfSpaceError::new("")))
        }

        todo!();
        
        if self.0[self.len(), 1] {
            return Err(Box::new(OutOfSpaceError::new("")));
        }

        for i in 0..self.0.len() {
            if self.0[i] > item {
                self.0.insert(i, item);
                return Ok(());
            }
        }

        Ok(())
    }

    pub fn remove(&mut self, index: usize) -> Option<T> {
        if self.0.len() <= index {
            return None;
        }

        self.0.remove(index)
    }

    pub fn pop(&mut self) -> Option<T> {
        todo!()
    }

    pub fn len(&self) -> usize {
        todo!()
    }
}

impl<T: Ord + Copy> Iterator for OrderedList<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        todo!()       
    }
}

#[cfg(tests)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(0, 0);
    }
}