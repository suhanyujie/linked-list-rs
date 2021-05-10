#[derive(PartialEq, Eq)]
enum Link {
    Empty,
    Elem(i32, Box<List>),
}

struct Node {
    elem: i32,
    next: List,
}

pub struct List {
    head: Link,
}

impl List {
    pub fn new() -> Self {
        List {
            head: (Link::Empty),
        }
    }

    pub fn push(&mut self, elem: i32) {
        let new_node = Node {
            elem,
            next: self.head,
        };
    }
}
