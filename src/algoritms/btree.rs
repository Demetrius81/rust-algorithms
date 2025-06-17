#![allow(dead_code)]
// #![allow(unused_variables)]
// #![allow(unused_assignments)]

use crate::algoritms::btree::btree::BinaryTree;

mod btree {
    use std::cell::RefCell;
    use std::collections::VecDeque;
    use std::rc::Rc;

    type NodeRef = Rc<RefCell<Node>>;

    #[derive(Debug)]
    pub struct Node {
        value: i32,
        left: Option<NodeRef>,
        right: Option<NodeRef>,
    }

    impl Node {
        pub fn new(value: i32) -> NodeRef {
            Rc::new(RefCell::new(Node {
                value,
                left: None,
                right: None,
            }))
        }
    }

    #[derive(Debug)]
    pub struct BinaryTree {
        root: Option<NodeRef>,
    }

    impl BinaryTree {
        pub fn new() -> Self {
            Self { root: None }
        }

        pub fn build_tree(&mut self, list: &Vec<i32>) {
            for item in list {
                self.add(*item);
            }
        }

        pub fn insert_node(&mut self, node: NodeRef) {
            if let Some(root) = &self.root {
                Self::insert_recursive(root.clone(), node);
            } else {
                self.root = Some(node);
            }
        }

        fn insert_recursive(current: NodeRef, new_node: NodeRef) {
            if new_node.borrow().value < current.borrow().value {
                if let Some(left_child) = &current.borrow().left {
                    Self::insert_recursive(left_child.clone(), new_node);
                } else {
                    current.borrow_mut().left = Some(new_node);
                }
            } else {
                if let Some(right_child) = &current.borrow().right {
                    Self::insert_recursive(right_child.clone(), new_node);
                } else {
                    current.borrow_mut().right = Some(new_node);
                }
            }
        }

        pub fn contains(&self, item: i32) -> bool {
            self.find_node(item).is_some()
        }

        pub fn find_node(&self, item: i32) -> Option<NodeRef> {
            Self::find_recursive(self.root.as_ref(), item)
        }

        fn find_recursive(node_opt: Option<&NodeRef>, item: i32) -> Option<NodeRef> {
            match node_opt {
                Some(node) => {
                    let node_borrow = node.borrow();
                    if node_borrow.value == item {
                        Some(Rc::clone(node))
                    } else if item < node_borrow.value {
                        Self::find_recursive(node_borrow.left.as_ref(), item)
                    } else {
                        Self::find_recursive(node_borrow.right.as_ref(), item)
                    }
                }
                None => None,
            }
        }

        pub fn add(&mut self, item: i32) {
            let new_node = Node::new(item);
            self.insert_node(new_node);
        }

        pub fn remove(&mut self, item: i32) -> Option<NodeRef> {
            Self::remove_recursive(None, self.root.as_ref(), item)
        }

        fn remove_recursive(
            parent_opt: Option<&NodeRef>,
            current_opt: Option<&NodeRef>,
            item: i32,
        ) -> Option<NodeRef> {
            match current_opt {
                Some(current) => {
                    let mut current_borrow = current.borrow_mut();
                    if item < current_borrow.value {
                        return Self::remove_recursive(
                            Some(current),
                            current_borrow.left.as_ref(),
                            item,
                        );
                    } else if item > current_borrow.value {
                        return Self::remove_recursive(
                            Some(current),
                            current_borrow.right.as_ref(),
                            item,
                        );
                    } else {
                        let replacement;
                        if current_borrow.left.is_none() && current_borrow.right.is_none() {
                            if let Some(parent) = parent_opt {
                                if parent
                                    .borrow()
                                    .left
                                    .as_ref()
                                    .map(|n| Rc::ptr_eq(n, current))
                                    .unwrap_or(false)
                                {
                                    parent.borrow_mut().left = None;
                                } else if parent
                                    .borrow()
                                    .right
                                    .as_ref()
                                    .map(|n| Rc::ptr_eq(n, current))
                                    .unwrap_or(false)
                                {
                                    parent.borrow_mut().right = None;
                                }
                            } else {
                                return Some(Rc::clone(current));
                            }
                            return Some(Rc::clone(current));
                        } else if current_borrow.left.is_none() || current_borrow.right.is_none() {
                            replacement = if current_borrow.left.is_some() {
                                Rc::clone(current_borrow.left.as_ref().unwrap())
                            } else {
                                Rc::clone(current_borrow.right.as_ref().unwrap())
                            };
                            if let Some(parent) = parent_opt {
                                if parent
                                    .borrow()
                                    .left
                                    .as_ref()
                                    .map(|n| Rc::ptr_eq(n, current))
                                    .unwrap_or(false)
                                {
                                    parent.borrow_mut().left = Some(Rc::clone(&replacement));
                                } else if parent
                                    .borrow()
                                    .right
                                    .as_ref()
                                    .map(|n| Rc::ptr_eq(n, current))
                                    .unwrap_or(false)
                                {
                                    parent.borrow_mut().right = Some(Rc::clone(&replacement));
                                }
                            } else {
                                return Some(replacement);
                            }
                            return Some(Rc::clone(current));
                        } else {
                            let min_node = Self::find_min(current_borrow.right.as_ref().unwrap());
                            let min_value = min_node.borrow().value;
                            current_borrow.value = min_value;
                            Self::remove_recursive(
                                Some(current),
                                current_borrow.right.as_ref(),
                                min_value,
                            )
                        }
                    }
                }
                None => None,
            }
        }

        pub fn find_min(node: &NodeRef) -> NodeRef {
            let mut current = Rc::clone(node);
            while let Some(left_child) = &Rc::clone(node).borrow().left {
                current = Rc::clone(left_child);
            }
            current
        }

        pub fn search(&self, target_value: i32) -> Option<NodeRef> {
            fn dfs(node_opt: &Option<NodeRef>, target_value: i32) -> Option<NodeRef> {
                match node_opt {
                    Some(node_rc) => {
                        if node_rc.borrow().value == target_value {
                            return Some(Rc::clone(node_rc));
                        }
                        dfs(&node_rc.borrow().left, target_value)
                            .or_else(|| dfs(&node_rc.borrow().right, target_value))
                    }
                    None => None,
                }
            }

            dfs(&self.root, target_value)
        }

        pub fn print_tree(&self) {
            if self.root.is_none() {
                println!("Дерево пустое");
                return;
            }

            let mut queue = VecDeque::new();
            queue.push_back((Rc::clone(self.root.as_ref().unwrap()), 0)); // кортеж (узел, уровень)
            let mut current_level = 0;

            while !queue.is_empty() {
                let (node, level) = queue.pop_front().unwrap();

                if level != current_level {
                    println!();
                    current_level = level;
                }

                print!("{} ", node.borrow().value);

                if let Some(left) = &node.borrow().left {
                    queue.push_back((Rc::clone(left), level + 1));
                }
                if let Some(right) = &node.borrow().right {
                    queue.push_back((Rc::clone(right), level + 1));
                }
            }
            println!();
        }
    }
}

pub fn run() {
    let v = vec![5, 7, 3, 6, 4, 8, 2, 9, 1];
    let mut tree = BinaryTree::new();
    tree.build_tree(&v);
    tree.print_tree();
}
