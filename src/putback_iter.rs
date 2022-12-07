/// This file implements a simple iterator that allows you to put back items.

use std::collections::VecDeque;

/// A simple iterator that allows you to put back items.
/// 
pub struct PutBack<I, T> {
    iter : I,
    buf  : VecDeque<T>,
}
impl<I: Iterator<Item=T>, T> PutBack<I, T> {
    pub fn new(iter: I) -> Self {
        Self { iter, buf: VecDeque::new() }
    }
    /// Put an item back into the iterator. It will be returned the next time
    /// next() is called.
    /// 
    pub fn put_back(&mut self, item: T) {
        self.buf.push_back(item);
    }
}
impl<I: Iterator<Item=T>, T> Iterator for PutBack<I, T> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        if !self.buf.is_empty() {
            self.buf.pop_front()
        } else {
            self.iter.next()
        }
    }
}
