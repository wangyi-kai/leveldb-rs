use std::cmp::Ordering::{Equal, Greater, Less};
use std::ops::Deref;
use std::ptr::NonNull;
use std::sync::atomic::{AtomicUsize, Ordering};

use crate::skiplist::node::Node;
use crate::utils::random::Random;
use crate::error::Result;
use crate::skiplist::error::SkipListError;

pub const MAX_HEIGHT: usize = 12;

pub trait Comparator<T> {
    fn cmp(&self, a: &T, b: &T) -> std::cmp::Ordering;     
}

pub struct SkipList<T, C: Comparator<T>> {
    pub(crate) head: NonNull<Node<T>>,
    compare: C,
    max_height: AtomicUsize,
}

impl<T, C: Comparator<T>> SkipList<T, C> {
    pub fn insert(&mut self, key: T) -> Result<()> {
        unsafe {
            let mut prev = vec![NonNull::new_unchecked(self.head.as_ptr()); MAX_HEIGHT];
            let mut x = self.find_greater_or_equal(&key, &mut prev);
            if x.is_none() {
                return Err(SkipListError::Fatal("没有可插入节点".to_string()).into());
            }
            if self.compare.cmp(x.unwrap().as_ref().key(), &key) == Equal {
                return Err(SkipListError::Fatal("不允许插入重复节点".to_string()).into());
            }
            let height = self.random_height()?;
            if height > self.get_max_height() {
                for i in self.get_max_height()..height {
                    prev[i] = self.head;
                }
                self.max_height.store(height, Ordering::Relaxed);
            }

            x = Some(NonNull::from_ref(&Node::new(key, height)));
            for i in 0..height {
                if let Some(node) = prev.get(i) {
                    x.unwrap().as_mut().no_barrier_set_next(i, Some(*node))?;
                } else {
                    x.unwrap().as_mut().no_barrier_set_next(i, None)?;
                }
                prev[i].as_mut().set_next(i, Some(NonNull::from_ref(x.unwrap().as_ref())))?;
            }
        }

        Ok(())
    }

    pub fn contains(&self, value: &T) -> bool {
        unsafe {
            let x = self.find_greater_or_equal(value, &mut vec![NonNull::new_unchecked(self.head.as_ptr()); MAX_HEIGHT]);
            x.map(|x| self.compare.cmp(x.as_ref().key(), value) == Equal).unwrap_or(false)
        }
    }

    pub fn get_max_height(&self) -> usize {
        self.max_height.load(Ordering::Relaxed)
    }

    pub fn random_height(&self) -> Result<usize> {
        let mut rnd = Random::init(0xdeadbeef);
        let mut height = 1;
        while height < MAX_HEIGHT && rnd.one_in(4){
            height += 1;
        }
        if height > MAX_HEIGHT {
            return Err(SkipListError::Fatal("生成的高度不满足要求".to_string()).into());
        }

        Ok(height)
    }

    pub fn key_is_after_node(&self, key: &T, node: Option<NonNull<Node<T>>>) -> bool {
        if let Some(node) = node {
            unsafe {
                self.compare.cmp(node.as_ref().key(), key) == Less
            }
        } else {
            false
        }
    }

    pub fn find_greater_or_equal(&self, key: &T, prev: &mut Vec<NonNull<Node<T>>>) -> Option<NonNull<Node<T>>> {
        let mut x = self.head;
        let mut level = self.get_max_height() - 1;
        loop {
            unsafe {
                let next = x.as_ref().next(level);
                if self.key_is_after_node(key, next) {
                    x = next.unwrap();
                } else {
                    prev[level] = x;
                    if level == 0 {
                        return next;
                    } else {
                        level -= 1;
                    }
                }
            }
        }
    }

    pub fn find_less_than(&self, key: &T) -> Option<NonNull<Node<T>>> {
        let mut x = self.head;
        let mut level = self.get_max_height() - 1;
        loop {
            unsafe {
                let next = x.as_ref().next(level);
                if next.is_none() || self.compare.cmp(next.unwrap().as_ref().key(), key) == Greater {
                    if level == 0 {
                        return Some(x);
                    } else {
                        level -= 1;
                    }
                } else {
                    x = next.unwrap();
                }
            }
        }
    }

    pub fn find_last(&self) -> Option<NonNull<Node<T>>> {
        let mut x = self.head;
        let mut level = self.get_max_height() - 1;
        loop {
            unsafe {
                let next = x.as_ref().next(level);
                if next.is_none() {
                    if level == 0 {
                        return Some(x);
                    } else {
                        level -= 1;
                    }
                } else {
                    x = next.unwrap();
                }
            }
        }
    }
}
