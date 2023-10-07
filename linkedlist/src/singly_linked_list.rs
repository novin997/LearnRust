use std::ops::{Deref, DerefMut};

pub struct SinglyLinkedList<T> {
    head: Option<Box<Node<T>>>,
    size: usize,
}

pub struct Node<T> {
    pub elem: T,
    next_node: Option<Box<Node<T>>>,
}

impl<T> SinglyLinkedList<T> {
    pub fn new() -> Self {
        SinglyLinkedList {
            head: None,
            size: 0,
        }
    }

    pub fn push_front(&mut self, t: T) {
        let node: Box<Node<T>> = Box::new(Node {
            elem: t,
            next_node: self.head.take(),
        });
        self.head = Some(node);
        self.size += 1;
    }

    pub fn pop_front(&mut self) -> Option<T> {
        self.head.take().map(|node: Box<Node<T>>| {
            self.head = node.next_node;
            self.size -= 1;
            node.elem
        })
    }

    // pub fn into_iter(self) -> IntoIter<T> {
    //     IntoIter(self)
    // }
}

// pub struct IntoIter<T>(SinglyLinkedList<T>);

impl<T> Iterator for Node<T> {
    type Item = Node<T>;
    fn next(&mut self) -> Option<Self::Item> {
        if let Some(x) = self.next_node {
            return Some(*x);
        }
        None
    }
}

impl<'a, T> IntoIterator for SinglyLinkedList<T> {
    type Item = Node<T>;
    type IntoIter = Node<T>;

    fn into_iter(self) -> Self::IntoIter {
        *self.head.unwrap()
    }
}
