use std::{collections::VecDeque, iter::Fuse};

pub struct MultiPeek<I>
where
    I: Iterator,
{
    iter: Fuse<I>,
    buf: VecDeque<I::Item>,
    index: usize,
}

pub fn multipeek<I>(iterable: I) -> MultiPeek<I::IntoIter>
where
    I: IntoIterator,
{
    MultiPeek {
        iter: iterable.into_iter().fuse(),
        buf: VecDeque::new(),
        index: 0,
    }
}

impl<I: Iterator> MultiPeek<I> {
    pub fn peek(&mut self) -> Option<&I::Item> {
        let ret = if self.index < self.buf.len() {
            Some(&self.buf[self.index])
        } else {
            match self.iter.next() {
                Some(x) => {
                    self.buf.push_back(x);
                    Some(&self.buf[self.index])
                }
                None => return None,
            }
        };

        self.index += 1;
        ret
    }
}

impl<I> Iterator for MultiPeek<I>
where
    I: Iterator,
{
    type Item = I::Item;

    fn next(&mut self) -> Option<Self::Item> {
        self.index = 0;
        self.buf.pop_front().or_else(|| self.iter.next())
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let (low, high) = self.iter.size_hint();
        let buf_len = self.buf.len();
        (low + buf_len, high.and_then(|h| h.checked_add(buf_len)))
    }

    fn fold<B, F>(self, mut init: B, mut f: F) -> B
    where
        F: FnMut(B, Self::Item) -> B,
    {
        init = self.buf.into_iter().fold(init, &mut f);
        self.iter.fold(init, f)
    }
}

impl<I> MultiPeek<I>
where
    I: Iterator,
{
    pub fn reset_peek(&mut self) {
        self.index = 0;
    }
}
