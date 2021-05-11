use std::mem;

#[derive(PartialEq, Eq)]
enum Link {
    Empty,
    More(Box<Node>),
}

#[derive(PartialEq, Eq)]
struct Node {
    elem: i32,
    next: Link,
}

#[derive(PartialEq, Eq)]
pub struct List {
    head: Link,
}

impl List {
    pub fn new() -> Self {
        List { head: Link::Empty }
    }

    pub fn push(&mut self, elem: i32) {
        let new_node = Box::new(Node {
            elem,
            next: mem::replace(&mut self.head, Link::Empty),
        });
        self.head = Link::More(new_node);
    }

    pub fn pop(&mut self) -> Option<i32> {
        match mem::replace(&mut self.head, Link::Empty) {
            Link::Empty => None,
            Link::More(boxed_node) => {
                let node = *boxed_node;
                self.head = node.next;
                Some(node.elem)
            }
        }
    }
}

impl Default for List {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_node1() {
        let mut list = List::new();
        assert_eq!(list.pop(), None);
    }
}
