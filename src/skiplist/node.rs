use std::ptr::NonNull;
use std::sync::atomic::{AtomicPtr, Ordering};

use crate::error::Result;
use crate::skiplist::error::SkipListError;
use crate::skiplist::skiplist::MAX_HEIGHT;

/// 跳表节点
pub struct Node<K> {
    key: K,
    next: Vec<AtomicPtr<Node<K>>>,
}

impl<K> Node<K> {
    pub fn new(key: K, height: usize) -> Self {
        let mut next = Vec::with_capacity(height);
        for _ in 0..height {
            next.push(AtomicPtr::default());
        }

        Node {
            key,
            next,
        }
    }

    pub fn key(&self) -> &K {
        &self.key
    }

    pub(crate) fn next(&self, level: usize) -> Option<NonNull<Node<K>>> {
        let ptr = self.next[level].load(Ordering::Acquire);
        unsafe {
            if ptr.is_null() {
                None
            } else {
                Some(NonNull::new_unchecked(ptr))
            }
        }
    }

    pub fn set_next(&mut self, level: usize, node: Option<NonNull<Node<K>>>) -> Result<()> {
        if level >= MAX_HEIGHT {
            return Err(SkipListError::Fatal("skiplist level > MAX_HEIGHT".to_string()).into());
        }
        let node = match node {
            Some(node) => node.as_ptr(),
            None => std::ptr::null_mut(),
        };
        self.next[level].store(node, Ordering::Release);

        Ok(())
    }

    pub fn no_barrier_next(&self, level: usize) -> Option<NonNull<Node<K>>> {
        let ptr = self.next[level].load(Ordering::Relaxed);
        unsafe {
            if ptr.is_null() {
                None
            } else {
                Some(NonNull::new_unchecked(ptr))
            }
        }
    }

    pub(crate) fn no_barrier_set_next(&mut self, level: usize, node: Option<NonNull<Node<K>>>) -> Result<()> {
        if level >= MAX_HEIGHT {
            return Err(SkipListError::Fatal("skiplist level > MAX_HEIGHT".to_string()).into());
        }
        let node = match node {
            Some(node) => node.as_ptr(),
            None => std::ptr::null_mut(),
        };
        self.next[level].store(node, Ordering::Relaxed);

        Ok(())
    }
}