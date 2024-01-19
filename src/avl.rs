use std::borrow::Borrow;
use std::fmt::{Debug, Formatter};

// TODO: Add permutation tests.
// TODO: Add iterator.
// TODO: Add readme.
// TODO: Update rebalance (and other functions) to only use &mut and not take. That should be all that's necessary.

struct Node<T> {
    left: Option<Box<Node<T>>>,
    right: Option<Box<Node<T>>>,
    value: T,
}

impl<T: Ord + Debug> Node<T> {
    fn new(value: T) -> Self {
        Self {
            left: None, right: None, value
        }
    }

    // Returns the node (a child) that should replace the node being removed.
    fn remove(&mut self) -> Option<Box<Node<T>>> {
        match (&mut self.left, &mut self.right) {
            (Some(_), Some(_)) => {
                let mut node = take_smallest(&mut self.right);
                match node {
                    None => {},
                    Some(ref mut node) => {
                        node.left = self.left.take();
                        node.right = self.right.take();
                    }
                }
                node
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
            Some(ref mut child_ref) => {
                if child_ref.value == value {
                    *child = child_ref.remove();
                } else {
                    child_ref.delete(value);
                }
                *child = rebalance(child.take());
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
            self.right = rebalance(self.right.take());
        } else if value < self.value {
            match self.left {
                None => self.left = Some(Box::new(Node::new(value))),
                Some(ref mut left) => left.insert(value),
            }
            self.left = rebalance(self.left.take());
        }
    }
}

pub struct Tree<T> {
    root: Option<Box<Node<T>>>
}

impl<T: Ord + Debug> Tree<T> {
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
                    self.root = root.remove();
                } else {
                    root.delete(value);
                }
            }
        }
        self.root = rebalance(self.root.take())
    }

    pub fn insert(&mut self, value: T) {
        match self.root {
            None => self.root = Some(Box::new(Node::new(value))),
            Some(ref mut node) => node.insert(value),
        }
        self.root = rebalance(self.root.take())
    }

    pub fn rotate_left(&mut self) {
        self.root = rotate_left(self.root.take());
    }

    pub fn rotate_right(&mut self) {
        self.root = rotate_right(self.root.take());
    }
}

fn balance_factor<T>(node: &Option<Box<Node<T>>>) -> i32 {
    match node {
        None => 0,
        Some(ref node) => height(&node.right) - height(&node.left),
    }
}

fn height<T>(node: &Option<Box<Node<T>>>) -> i32 {
    match node {
        None => 0,
        Some(ref node) => height(&node.left).max(height(&node.right)) + 1,
    }
}

fn take_smallest<T>(node: &mut Option<Box<Node<T>>>) -> Option<Box<Node<T>>> {
    let mut current = node;
    while let Some(node) = current {
        current = &mut node.left;
    }
    let mut result = current.take();
    match result {
        None => {},
        Some(ref mut result) => {
            *current = result.right.take();
        }
    }
    result
}

fn rotate_right<T>(mut root: Option<Box<Node<T>>>) -> Option<Box<Node<T>>> {
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

fn rotate_left<T>(mut root: Option<Box<Node<T>>>) -> Option<Box<Node<T>>> {
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

fn rebalance<T>(mut node: Option<Box<Node<T>>>) -> Option<Box<Node<T>>> {
    let bf = balance_factor(&node);
    let new_node = match node {
        None => None,
        Some(ref mut node_ref) => {
            if bf > 1 {
                // Right tree is too tall.
                let rbf = balance_factor(&node_ref.right);
                assert!(rbf >= -1 && rbf <= 1);
                if rbf == -1 {
                    // Right tree only has a left child (no right child). If we only rotate_left, the height of the tree won't change.
                    node_ref.right = rotate_right(node_ref.right.take());
                }
                rotate_left(node)
            } else if bf < -1 {
                // Left tree is too tall.
                let lbf = balance_factor(&node_ref.left);
                assert!(lbf >= -1 && lbf <= 1);
                if lbf == 1 {
                    // Left tree only has a right child (no left child). If we only rotate_right, the height of the tree won't change.
                    node_ref.left = rotate_left(node_ref.left.take());
                }
                rotate_right(node)
            } else {
                // Tree is balanced.
                node
            }
        }
    };
    let bf = balance_factor(&new_node);
    assert!(bf >= -1 && bf <= 1);
    new_node
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

    fn assert_tree_invariants<T: Ord>(node: &Option<Box<Node<T>>>) {
        let bf = balance_factor(&node);
        assert!(bf >= -1 && bf <= 1);
        match node {
            None => {},
            Some(ref node) => {
                match node.left {
                    None => {},
                    Some(ref left) => assert!(left.value < node.value),
                }
                match node.right {
                    None => {},
                    Some(ref right) => assert!(right.value > node.value),
                }
            }
        }
    }

    #[test]
    fn insert_basic() {
        let mut tree = Tree::new();
        tree.insert(5);
        assert_tree_invariants(&tree.root);
        assert!(tree.contains(5));
        assert!(!tree.contains(4));
    }

    #[test]
    fn many_inserts_and_deletes() {
        let mut tree = Tree::new();
        for i in 0..100 {
            tree.insert(i);
            assert_tree_invariants(&tree.root);
        }
        for i in 0..100 {
            assert!(tree.contains(i));
        }
        for i in 0..100 {
            tree.delete(i);
            assert_tree_invariants(&tree.root);
            assert!(!tree.contains(i));
        }
    }

    #[test]
    fn delete_edge_case() {
        let mut tree = Tree::new();
        tree.insert(10);
        tree.insert(20);
        tree.insert(15);
        tree.insert(17);
        tree.delete(10);
        assert!(tree.contains(20));
        assert!(tree.contains(15));
        assert!(tree.contains(17));
    }

    #[test]
    fn two_node_left_rotate() {
        let mut tree = Tree::new();
        tree.insert(1);
        tree.insert(2);
        assert_eq!(tree.root.as_ref().unwrap().value, 1);
        assert!(tree.contains(2));
        assert_tree_invariants(&tree.root);
        tree.rotate_left();
        assert_tree_invariants(&tree.root);
        assert_eq!(tree.root.as_ref().unwrap().value, 2);
        assert!(tree.contains(1));
    }

    #[test]
    fn two_node_right_rotate() {
        let mut tree = Tree::new();
        tree.insert(2);
        tree.insert(1);
        assert_eq!(tree.root.as_ref().unwrap().value, 2);
        assert!(tree.contains(1));
        assert_tree_invariants(&tree.root);
        tree.rotate_right();
        assert_tree_invariants(&tree.root);
        assert_eq!(tree.root.as_ref().unwrap().value, 1);
        assert!(tree.contains(2));
    }

    #[test]
    fn rotate_right_left_insertion() {
        let mut tree = Tree::new();
        tree.insert(1);
        tree.insert(3);
        tree.insert(2);
        assert_tree_invariants(&tree.root);
        assert_eq!(height(&tree.root), 2);
    }

    #[test]
    fn rotate_left_insertion() {
        let mut tree = Tree::new();
        tree.insert(1);
        tree.insert(2);
        tree.insert(3);
        assert_tree_invariants(&tree.root);
        assert_eq!(height(&tree.root), 2);
    }

    #[test]
    fn rotate_right_insertion() {
        let mut tree = Tree::new();
        tree.insert(3);
        tree.insert(2);
        tree.insert(1);
        assert_tree_invariants(&tree.root);
        assert_eq!(height(&tree.root), 2);
    }

    #[test]
    fn rotate_left_right_insertion() {
        let mut tree = Tree::new();
        tree.insert(3);
        tree.insert(1);
        tree.insert(2);
        assert_tree_invariants(&tree.root);
        assert_eq!(height(&tree.root), 2);
    }
}
