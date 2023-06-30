pub mod priority_queue {
    pub struct PriorityQueue<T> {
        priority: Vec<i32>,
        queue: Vec<T>
    }

    impl<T> PriorityQueue<T> {
        pub fn new() -> Self {
            PriorityQueue { 
                priority: Vec::new(),
                queue: Vec::new()
            }
        }

        pub fn put(&mut self, itemPriority:i32, item: T) {
            if self.priority.len() == 0 {
                self.priority.push(itemPriority);
                self.queue.push(item);
                return ();
            }

            if self.priority[0] <= itemPriority {
                self.priority.insert(0, itemPriority);
                self.queue.insert(0, item);
                return ();
            }

            for priority_idx in (0..self.priority.len()).rev() {
                if self.priority[priority_idx] <= itemPriority {
                    self.priority.insert(priority_idx, itemPriority);
                    self.queue.insert(priority_idx, item);
                    return ();
                }
            }

            self.priority.push(itemPriority);
            self.queue.push(item);
        }

        pub fn pop(&mut self) -> Option<(i32, T)> {
            let prio = match self.priority.pop() {
                Some(v) => v,
                None => return None
            };
            let value = match self.queue.pop() {
                Some(v) => v,
                None => return None
            };

            return Some((prio, value));
        }

        pub fn len(&self) -> usize {
            return self.queue.len();
        }
    }
}