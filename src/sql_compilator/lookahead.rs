use std::collections::VecDeque;

/// Structure implementing a peekable Iterator, with the ability to peek further than
/// only the first element
pub struct Lookahead<I: Iterator> {
    iter: I,
    buf: VecDeque<I::Item>,
}

impl<I: Iterator> Lookahead<I> {
    pub fn new(iter: I) -> Self {
        Lookahead {
            iter,
            buf: VecDeque::new(),
        }
    }

    /// Returns a reference to the n-th element of the original iterator
    pub fn peek(&mut self, n: usize) -> Option<&I::Item> {
        while self.buf.len() < n {
            if let Some(item) = self.iter.next() {
                self.buf.push_back(item);
            } else {
                break;
            }
        }
        self.buf.get(n)
    }
}

/// Allows the use of a .lookahead() method on a sized iterator
pub trait LookaheadExt: Iterator + Sized {
    fn lookahead(self) -> Lookahead<Self> {
        Lookahead::new(self)
    }
}

impl<I: Iterator> LookaheadExt for I {}

impl<I: Iterator> Iterator for Lookahead<I> {
    type Item = I::Item;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(front) = self.buf.pop_front() {
            Some(front)
        } else {
            self.iter.next()
        }
    }
}
