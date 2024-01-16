use std::fmt::{Debug, Formatter};

// TODO: Add an iterator (that supports reverse)?
// TODO: Add various test cases, including fuzzing.

struct Node<T> {
    left: Option<Box<Node<T>>>,
    right: Option<Box<Node<T>>>,
    value: T,
}

impl<T: Ord> Node<T> {
    fn new(value: T) -> Self {
        Self {
            left: None, right: None, value
        }
    }

    fn is_leaf(&self) -> bool {
        self.left.is_none() && self.right.is_none()
    }

    fn take_smallest(&mut self) -> Box<Node<T>> {
        let mut current = &mut self.left;
        while let Some(node) = current {
            current = &mut node.left;
        }
        match current.take() {
            None => panic!("take_smallest called on a node with no left child."),
            Some(node) => node,
        }
    }

    // Returns the value that should replace removing a child node.
    fn remove(&mut self) -> Option<Box<Node<T>>> {
        match (&mut self.left, &mut self.right) {
            (Some(_), Some(ref mut right)) => {
                let mut node = if right.is_leaf() {
                    self.right.take().unwrap()
                } else {
                    right.take_smallest()
                };
                node.left = self.left.take();
                node.right = self.right.take();
                // println!("lifetime failure test: {:?}", left);
                Some(node)
            },
            (None, Some(_)) => self.right.take(),
            (Some(_), None) => self.left.take(),
            (None, None) => None,
        }
    }

    fn delete(&mut self, value: T) {
        let child = if value < self.value {
            &mut self.left
        } else {
            &mut self.right
        };
        match child {
            None => {}
            Some(ref mut node) => {
                if node.value == value {
                    *child = node.remove();
                } else {
                    node.delete(value);
                }
            }
        }
    }

    fn contains(&self, value: T) -> bool {
        if value == self.value {
            true
        } else if value > self.value {
            match self.right {
                None => false,
                Some(ref right) => right.contains(value),
            }
        } else {
            match self.left {
                None => false,
                Some(ref left) => left.contains(value),
            }
        }
    }

    fn insert(&mut self, value: T) {
        if value > self.value {
            match self.right {
                None => self.right = Some(Box::new(Node::new(value))),
                Some(ref mut right) => right.insert(value),
            }
        } else if value < self.value {
            match self.left {
                None => self.left = Some(Box::new(Node::new(value))),
                Some(ref mut left) => left.insert(value),
            }
        }
    }
}

pub struct Tree<T> {
    root: Option<Box<Node<T>>>
}

impl<T: Ord> Tree<T> {
    pub fn new() -> Self {
        Self {
            root: None
        }
    }

    pub fn contains(&self, value: T) -> bool {
        match self.root {
            None => false,
            Some(ref node) => node.contains(value),
        }
    }

    pub fn delete(&mut self, value: T) {
        match self.root {
            None => {},
            Some(ref mut root) => {
                if root.value == value {
                    self.root = None;
                } else {
                    root.delete(value);
                }
            }
        }
    }

    pub fn insert(&mut self, value: T) {
        match self.root {
            None => self.root = Some(Box::new(Node::new(value))),
            Some(ref mut node) => node.insert(value),
        }
    }

    pub fn rotate_left(&mut self) {
        self.root = rotate_left(self.root.take());
    }

    pub fn rotate_right(&mut self) {
        self.root = rotate_right(self.root.take());
    }
}

fn rotate_right<T: Ord>(mut root: Option<Box<Node<T>>>) -> Option<Box<Node<T>>> {
    match root {
        None => None,
        Some(ref mut root_ref) => {
            let mut pivot = root_ref.left.take();
            match pivot {
                None => root,
                Some(ref mut pivot_ref) => {
                    let transfer = pivot_ref.right.take();
                    root_ref.left = transfer;
                    pivot_ref.right = root;
                    pivot
                }
            }
        }
    }
}

fn rotate_left<T: Ord>(mut root: Option<Box<Node<T>>>) -> Option<Box<Node<T>>> {
    match root {
        None => None,
        Some(ref mut root_ref) => {
            let mut pivot = root_ref.right.take();
            match pivot {
                None => root,
                Some(ref mut pivot_ref) => {
                    let transfer = pivot_ref.left.take();
                    root_ref.right = transfer;
                    pivot_ref.left = root;
                    pivot
                }
            }
        }
    }
}

impl<T: Debug> Debug for Node<T> {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "Node(value: {:?}, left: {:?}, right: {:?})", self.value, self.left, self.right)
    }
}

impl<T: Debug> Debug for Tree<T> {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "Tree({:?})", self.root)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn insert_basic() {
        let mut tree = Tree::new();
        tree.insert(5);
        assert!(tree.contains(5));
        assert!(!tree.contains(4));
    }

    #[test]
    fn many_inserts() {
        let mut tree = Tree::new();
        for i in 0..100 {
            tree.insert(i);
        }
        for i in 0..100 {
            assert!(tree.contains(i));
        }
    }
}
