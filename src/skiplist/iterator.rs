use std::ptr::NonNull;
use crate::skiplist::node::Node;
use crate::skiplist::skiplist::{Comparator, SkipList};

pub struct SkipListIterator<'a, T, C: Comparator<T>> {
    skip_list: &'a SkipList<T, C>,
    node: Option<NonNull<Node<T>>>,
}

impl <'a, T, C: Comparator<T>> SkipListIterator<'a, T, C> {
    pub fn valid(&self) -> bool {
        self.node.is_some()
    }

    pub fn key(&self) -> Option<&T> {
        self.node.map(|node| unsafe { node.as_ref().key() })
    }

    pub fn next(&mut self) {
        assert!(self.valid());
        unsafe {
            if let Some(node) = self.node {
                self.node = node.as_ref().next(0);
            }
        }
    }

    pub fn prev(&mut self) {
        assert!(self.valid());
        unsafe {
            let node = self.skip_list.find_less_than(self.node.unwrap().as_ref().key());
            if let Some(node) = node {
                if self.skip_list.head == node {
                    self.node = None;
                }
            }
        }
    }

    pub fn seek(&mut self, target: &T) {
        self.node = self.skip_list.find_greater_or_equal(target, &mut Vec::new());
    }
}