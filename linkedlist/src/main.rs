use std::ops::{Deref, DerefMut};

use crate::singly_linked_list::SinglyLinkedList;

mod singly_linked_list;

fn main() {
    let mut list: SinglyLinkedList<i32> = SinglyLinkedList::new();
    list.push_front(1);
    list.push_front(2);
    let mut iter = list.into_iter();
    println!("{}", iter.elem);
    iter.next();
    println!("{}", iter.elem);

    // if let Some(mut x) = list.head {
    //     if let Some(y) = x.deref_mut().next() {
    //         println!("{}", y);
    //     }
    // }

    // if let Some(x) = list.pop_front() {
    //     println!("{}", x);
    // }
    // if let Some(x) = list.pop_front() {
    //     println!("{}", x);
    // }
}
