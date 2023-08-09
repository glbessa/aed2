

pub struct Tst {
    head: Option<Box<TstNode>>
}

struct TstNode {
    terminal: bool,
    left: Option<Box<TstNode>>,
    center: Option<Box<TstNode>>,
    right: Option<Box<TstNode>>,
    value: Option<char>
}

impl<T> Tst<T> {
    pub fn new() -> Self {
        Tst {
            head: Some(Box::new(
                TstNode {
                    left: None,
                    center: None,
                    right: None,
                    terminal: false,
                    value: None
                }
            ))
        }
    }

    /*
    pub fn insert(&mut self, data: String) {
        let mut t: String = String::new();
        data.into_iter().map(|d| t.push(d));
        
        match self.head {
            Some(ref mut node) => {
                self.head = Some(node)
            },
            None => {
                let mut actual_node: Box<TstNode> = Box::new(TstNode::new());
                actual_node.value = Some(t.pop());
                actual_node.center = Some(Box::new());
                actual_node.
                self.head = Some(actual_node);

                loop {
                    actual_node = Box::new(TstNode::new());
                    
                    if t.len() == 0 {
                        actual_node.terminal = true;
                    }
                }
            }
        };
        let actual = self.head;
    }*/

    pub fn remove(&mut self, data: String) {
        todo!();
    }

    pub fn contains(&self, data: String) -> bool {
        todo!();
    }

    pub fn search(&self, data: String) -> Vec<String> {
        todo!();
    }
}

impl TstNode {
    fn new() -> Self {
        TstNode {
            left: None,
            center: None,
            right: None,
            terminal: false,
            value: None,
        }
    }
}