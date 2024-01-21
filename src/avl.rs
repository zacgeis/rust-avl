use std::fmt::{Debug, Formatter};

struct Node<T> {
    left: Option<Box<Node<T>>>,
    right: Option<Box<Node<T>>>,
    value: T,
}

impl<T> Node<T> {
    fn new(value: T) -> Self {
        Self {
            left: None, right: None, value
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

    pub fn insert(&mut self, value: T) {
        insert(&mut self.root, value);
    }

    pub fn contains(&self, value: &T) -> bool {
        contains(&self.root, value)
    }

    pub fn delete(&mut self, value: &T) {
        delete(&mut self.root, value);
    }
}

fn insert<T: Ord>(node: &mut Option<Box<Node<T>>>, value: T) {
    match node {
        None => *node = Some(Box::new(Node::new(value))),
        Some(ref mut node_value) => {
            if value < node_value.value {
                insert(&mut node_value.left, value);
            } else if value > node_value.value {
                insert(&mut node_value.right, value);
            }
            rebalance(node);
        }
    };
}

fn contains<T: Eq + Ord>(node: &Option<Box<Node<T>>>, value: &T) -> bool {
    match node {
        None => false,
        Some(ref node) => {
            if value == &node.value {
                true
            } else if value < &node.value {
                contains(&node.left, value)
            } else {
                contains(&node.right, value)
            }
        }
    }
}

fn take_smallest<T>(node: &mut Option<Box<Node<T>>>) -> Option<Box<Node<T>>> {
    match node {
        None => None,
        Some(ref mut node_value) => {
            match node_value.left {
                None => {
                    let replacement = node_value.right.take();
                    let result = node.take();
                    *node = replacement;
                    result
                }
                Some(_) => take_smallest(&mut node_value.left),
            }
        }
    }
}

fn delete<T: Ord + Eq>(node: &mut Option<Box<Node<T>>>, value: &T) {
    match node {
        None => {},
        Some(ref mut node_ref) => {
            if value == &node_ref.value {
                let mut next = take_smallest(&mut node_ref.right);
                match next {
                    None => *node = node_ref.left.take(),
                    Some(ref mut next_ref) => {
                        next_ref.left = node_ref.left.take();
                        next_ref.right = node_ref.right.take();
                        *node = next;
                    }
                }
            } else if value < &node_ref.value {
                delete(&mut node_ref.left, value)
            } else {
                delete(&mut node_ref.right, value)
            }
            rebalance(node);
        }
    };
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

fn rotate_right<T>(root: &mut Option<Box<Node<T>>>) {
    match root {
        None => {},
        Some(ref mut root_ref) => {
            let mut pivot = root_ref.left.take();
            match pivot {
                None => {},
                Some(ref mut pivot_ref) => {
                    root_ref.left = pivot_ref.right.take();
                    pivot_ref.right = root.take();
                }
            }
            *root = pivot;
        }
    };
}

fn rotate_left<T>(root: &mut Option<Box<Node<T>>>) {
    match root {
        None => {},
        Some(ref mut root_ref) => {
            let mut pivot = root_ref.right.take();
            match pivot {
                None => {},
                Some(ref mut pivot_ref) => {
                    root_ref.right = pivot_ref.left.take();
                    pivot_ref.left = root.take();
                }
            }
            *root = pivot;
        }
    };
}

fn rebalance<T>(node: &mut Option<Box<Node<T>>>) {
    let bf = balance_factor(node);
    match node {
        None => {},
        Some(ref mut node_ref) => {
            if bf > 1 {
                // Right tree is too tall.
                let rbf = balance_factor(&node_ref.right);
                assert!(rbf >= -1 && rbf <= 1);
                if rbf == -1 {
                    // Right tree only has a left child (no right child). If we only rotate_left, the height of the tree won't change.
                    rotate_right(&mut node_ref.right);
                }
                rotate_left(node)
            } else if bf < -1 {
                // Left tree is too tall.
                let lbf = balance_factor(&node_ref.left);
                assert!(lbf >= -1 && lbf <= 1);
                if lbf == 1 {
                    // Left tree only has a right child (no left child). If we only rotate_right, the height of the tree won't change.
                    rotate_left(&mut node_ref.left);
                }
                rotate_right(node)
            }
        }
    };
    let bf = balance_factor(node);
    assert!(bf >= -1 && bf <= 1);
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
        assert!(tree.contains(&5));
        assert!(!tree.contains(&4));
    }

    #[test]
    fn many_inserts_and_deletes() {
        let mut tree = Tree::new();
        for i in 0..100 {
            tree.insert(i);
            assert_tree_invariants(&tree.root);
        }
        for i in 0..100 {
            assert!(tree.contains(&i));
        }
        for i in 0..100 {
            tree.delete(&i);
            assert_tree_invariants(&tree.root);
            assert!(!tree.contains(&i));
        }
    }

    #[test]
    fn delete_edge_case() {
        let mut tree = Tree::new();
        tree.insert(10);
        tree.insert(20);
        tree.insert(15);
        tree.insert(17);
        tree.delete(&10);
        assert!(tree.contains(&20));
        assert!(tree.contains(&15));
        assert!(tree.contains(&17));
    }

    #[test]
    fn two_node_left_rotate() {
        let mut tree = Tree::new();
        tree.insert(1);
        tree.insert(2);
        assert_eq!(tree.root.as_ref().unwrap().value, 1);
        assert!(tree.contains(&2));
        assert_tree_invariants(&tree.root);
        rotate_left(&mut tree.root);
        assert_tree_invariants(&tree.root);
        assert_eq!(tree.root.as_ref().unwrap().value, 2);
        assert!(tree.contains(&1));
    }

    #[test]
    fn two_node_right_rotate() {
        let mut tree = Tree::new();
        tree.insert(2);
        tree.insert(1);
        assert_eq!(tree.root.as_ref().unwrap().value, 2);
        assert!(tree.contains(&1));
        assert_tree_invariants(&tree.root);
        rotate_right(&mut tree.root);
        assert_tree_invariants(&tree.root);
        assert_eq!(tree.root.as_ref().unwrap().value, 1);
        assert!(tree.contains(&2));
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
