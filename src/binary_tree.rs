//! Rust 实现二叉查找树
//! 参考：https://rustcc.gitbooks.io/rustprimer/content/data-structure/binary_tree.html

use std::error::Error;

type TreeNode<K, V> = Option<Box<Node<K, V>>>;

struct BinTree<K: PartialOrd, V> {
    root: TreeNode<K, V>,
}

struct Node<K: PartialOrd, V> {
    key: K,
    value: V,
    left: TreeNode<K, V>,
    right: TreeNode<K, V>,
}

impl<K, V> BinTree<K, V>
where
    K: PartialOrd,
{
    pub fn new() -> Self {
        BinTree { root: None }
    }

    pub fn insert(&mut self, key: K, value: V) -> Result<(), String> {
        if self.root.is_none() {
            let new_node = Node::new(key, value);
            self.root = Some(Box::new(new_node));
            return Ok(());
        }
        if let Some(res) = self.root.as_mut().map(|node| node.insert(key, value)) {
            res
        } else {
            Err("insert error.".into())
        }
    }

    pub fn search(&self, key: K) -> Option<&V> {
        if let Some(res) = self.root.as_ref().map(|node| node.search(key)) {
            res
        } else {
            None
        }
    }

    // 删除节点 todo
    pub fn delete(&mut self, key: K) -> Result<(), String> {
        if self.root.is_none() {
            return Ok(());
        }
        if let Some(res) = self.root.as_mut().map(|node| node.delete(key)) {
            res
        } else {
            Err("delete error.".into())
        }
    }
}

impl<K, V> Node<K, V>
where
    K: PartialOrd,
{
    pub fn new(key: K, value: V) -> Self {
        Node {
            key,
            value,
            left: None,
            right: None,
        }
    }

    // 插入新元素
    pub fn insert(&mut self, key: K, value: V) -> Result<(), String> {
        if key < self.key {
            if let Some(ref mut left) = self.left {
                return left.insert(key, value);
            } else {
                self.left = Some(Box::new(Node::new(key, value)));
            }
        } else {
            match self.right {
                Some(ref mut right) => {
                    return right.insert(key, value);
                }
                None => {
                    self.right = Some(Box::new(Node::new(key, value)));
                }
            }
        }

        Ok(())
    }

    /// 删除节点。删除的原理参考：https://blog.csdn.net/zhizhengguan/article/details/106624336
    /// 1删除的节点有两子节点
    /// 2删除的节点有左节点
    /// 3删除的节点有右节点
    pub fn delete(&mut self, key: K) -> Result<(), String> {
        if self.key == key {
            if self.left.is_none() && self.right.is_none() {
                todo!();
            } else if self.left.is_some() && self.right.is_some() {
                todo!();
            } else if self.left.is_none() {
                todo!();
            } else if self.right.is_none() {
                todo!();
            }
        }
        todo!();
        Ok(())
    }

    pub fn search(&self, key: K) -> Option<&V> {
        if self.key == key {
            return Some(&self.value);
        } else if self.key < key {
            if self.right.is_none() {
                None
            } else {
                self.right.as_ref().and_then(|node| {
                    return node.search(key);
                })
            }
        } else {
            if self.left.is_none() {
                None
            } else {
                self.left.as_ref().and_then(|node| {
                    return node.search(key);
                })
            }
        }
    }
}

mod tests {
    use super::*;

    #[test]
    fn test_insert() {
        let mut t1: BinTree<&str, &str> = BinTree::new();
        t1.insert("name", "LiuDeHua");
        t1.insert("age", "12");
        assert_eq!(t1.search("age"), Some(&"12"));
    }
}
