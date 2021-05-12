type Link<T> = Option<Box<Node<T>>>;

#[derive(PartialEq, Eq)]
struct Node<T> {
    elem: T,
    next: Link<T>,
}

#[derive(PartialEq, Eq)]
pub struct List<T> {
    head: Link<T>,
}

impl<T> List<T> {
    pub fn new() -> Self {
        List { head: None }
    }

    pub fn push(&mut self, elem: T) {
        let new_node = Box::new(Node {
            elem,
            next: self.head.take(),
        });
        self.head = Some(new_node);
    }

    pub fn pop(&mut self) -> Option<T> {
        self.head.take().map(|node| {
            let node = *node;
            self.head = node.next;
            node.elem
        })
    }

    pub fn peek(&self) -> Option<&T> {
        self.head.as_ref().map(|node| &node.elem)
    }

    pub fn peek_mut(&mut self) -> Option<&mut T> {
        self.head.as_mut().map(|node| &mut node.elem)
    }
}

impl<T> Default for List<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> Drop for List<T> {
    fn drop(&mut self) {
        let mut cur_link = self.head.take();
        while let Some(mut boxed_node) = cur_link {
            cur_link = boxed_node.next.take();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_node1() {
        let mut list = List::new();
        list.push(12);
        assert_eq!(list.pop(), Some(12));
    }

    #[test]
    fn test_peek() {
        let mut list = List::new();
        list.push("User1");
        list.push("Age: 22");
        assert_eq!(list.peek(), Some(&"Age: 22"));
        assert_eq!(list.peek_mut(), Some(&mut "Age: 22"));
    }
}
