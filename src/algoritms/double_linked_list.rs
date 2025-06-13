#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_assignments)]

use std::cell::RefCell;
use std::fmt::Debug;
use std::rc::Rc;

// Структура узла
#[derive(Debug)]
pub struct Node {
    pub value: i64,
    pub prev: Option<Rc<RefCell<Node>>>,
    pub next: Option<Rc<RefCell<Node>>>,
}

impl Node {
    fn new(value: i64) -> Rc<RefCell<Node>> {
        Rc::new(RefCell::new(Node {
            value,
            prev: None,
            next: None,
        }))
    }
}

#[derive(Debug)]
pub struct DList {
    head: Option<Rc<RefCell<Node>>>,
    tail: Option<Rc<RefCell<Node>>>,
    count: usize,
}

impl DList {
    pub fn new() -> Self {
        Self {
            head: None,
            tail: None,
            count: 0,
        }
    }

    pub fn get_count(&self) -> usize {
        self.count
    }

    pub fn add_node(&mut self, value: i64) -> Rc<RefCell<Node>> {
        let new_node = Node::new(value);
        match self.tail.take() {
            Some(old_tail) => {
                old_tail.borrow_mut().next = Some(Rc::clone(&new_node));
                new_node.borrow_mut().prev = Some(old_tail);
                self.tail = Some(new_node.clone());
            }
            None => {
                self.head = Some(new_node.clone());
                self.tail = Some(new_node.clone());
            }
        }
        self.count += 1;
        new_node
    }

    pub fn add_node_after(&mut self, node: Rc<RefCell<Node>>, value: i64) -> Rc<RefCell<Node>> {
        let new_node = Node::new(value);
        let mut node_borrow = node.borrow_mut();

        match node_borrow.next.take() {
            Some(next_node) => {
                new_node.borrow_mut().prev = Some(Rc::clone(&node));
                new_node.borrow_mut().next = Some(Rc::clone(&next_node));
                next_node.borrow_mut().prev = Some(Rc::clone(&new_node));
                node_borrow.next = Some(new_node.clone());
            }
            None => {
                new_node.borrow_mut().prev = Some(Rc::clone(&node));
                node_borrow.next = Some(Rc::clone(&new_node));
                self.tail = Some(new_node.clone());
            }
        }
        self.count += 1;
        new_node
    }

    pub fn remove_node_by_index(&mut self, index: usize) {
        if index >= self.count {
            println!("Index out of range.");
            return;
        }

        let current_opt = self.head.as_ref().map(Rc::clone);
        for _ in 0..index {
            match current_opt {
                Some(ref current) => {
                    let mut current_1 = self.head.as_ref().map(Rc::clone);
                    current_1 = current.borrow().next.as_ref().map(Rc::clone);
                }
                None => {
                    println!("Node not found.");
                    return;
                }
            }
        }

        if let Some(node) = current_opt {
            self.remove_node_by_node(node);
        }
    }

    pub fn remove_node_by_node(&mut self, node: Rc<RefCell<Node>>) {
        let prev_opt = node.borrow().prev.as_ref().map(Rc::clone);
        let next_opt = node.borrow().next.as_ref().map(Rc::clone);

        if let Some(prev) = prev_opt.as_ref() {
            prev.borrow_mut().next = next_opt.as_ref().map(Rc::clone);
        } else {
            self.head = next_opt.as_ref().map(Rc::clone);
        }

        if let Some(next) = next_opt.as_ref() {
            next.borrow_mut().prev = prev_opt.as_ref().map(Rc::clone);
        } else {
            self.tail = prev_opt.as_ref().map(Rc::clone);
        }

        self.count -= 1;
    }

    pub fn find_node_by_value(&self, search_value: i64) -> Option<Rc<RefCell<Node>>> {
        let mut current_opt = self.head.as_ref().map(Rc::clone);
        while let Some(current) = current_opt {
            if current.borrow().value == search_value {
                return Some(current);
            }
            current_opt = current.borrow().next.as_ref().map(Rc::clone);
        }
        None
    }
}

pub fn run() {
    let mut dlist = DList::new();
    let _ = dlist.add_node(1);
    let n2 = dlist.add_node(2);
    let n3 = dlist.add_node(3);
    println!("add 3 nodes > {:#?}", dlist.get_count());
    let _ = dlist.add_node_after(n2, 22);
    let n22 = dlist.add_node_after(n3, 33);
    println!("add 2 nodes after > {:?}", dlist.get_count());
    let v2 = dlist.find_node_by_value(22);
    println!("find node by value 22 > {:?}", v2.unwrap().borrow().value);
    let v1 = dlist.find_node_by_value(33);
    println!("find node by value 33 > {:?}", v1.unwrap().borrow().value);
    dlist.remove_node_by_node(n22);
    println!("remove node > {:?}", dlist.get_count());
    dlist.remove_node_by_index(3);
    println!("remove node by idx > {:?}", dlist.get_count());
}
