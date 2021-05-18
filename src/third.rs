use std::rc::Rc;

type Link<T> = Option<Rc<Node<T>>>;

#[derive(PartialEq, Eq, Debug)]
struct Node<T> {
    elem: T,
    next: Link<T>,
}

#[derive(PartialEq, Eq, Debug)]
pub struct List<T> {
    head: Link<T>,
}

impl<T> List<T> {
    pub fn new() -> Self {
        List { head: None }
    }

    pub fn push(&mut self, elem: T) {
        let new_node = Rc::new(Node {
            elem,
            next: self.head.take(),
        });
        self.head = Some(new_node);
    }

    // 新增方法 append
    pub fn append(&self, elem: T) -> List<T> {
        List {
            head: Some(Rc::new(Node {
                elem: elem,
                next: self.head.clone(),
            })),
        }
    }

    // 新增方法 head
    pub fn head(&self) -> Option<&T> {
        self.head.as_ref().map(|node| &node.elem)
    }

    // 新增方法 tail
    pub fn tail(&self) -> List<T> {
        List {
            head: self.head.as_ref().and_then(|node| node.next.clone()),
        }
    }
}

mod tests {
    use super::*;

    #[test]
    fn test_append() {
        let list: List<i32> = List::new();
        assert_eq!(list.head(), None);
        let list = list.append(3);
        assert_eq!(list.head(), Some(&3));
    }
}
