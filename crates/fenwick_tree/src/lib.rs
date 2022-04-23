pub use algebraic_structures::abelian_group::AbelianGroup;
use std::ops::RangeBounds;

#[derive(Debug, Clone)]
pub struct FenwickTree<A: AbelianGroup> {
    len: usize,
    buffer: Vec<A>,
}

impl<A: AbelianGroup> From<Vec<A::T>> for FenwickTree<A> {
    fn from(v: Vec<A::T>) -> Self {
        let mut res = FenwickTree::new(v.len());
        for (i, val) in v.into_iter().enumerate() {
            res.add(i, val);
        }
        res
    }
}

impl<A: AbelianGroup> FenwickTree<A> {
    pub fn new(len: usize) -> Self {
        Self {
            len: len + 1,
            buffer: vec![A::identity(); len + 1],
        }
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    fn prefix_inner(&self, to: usize) -> A {
        let mut res = self.buffer[0].clone();
        let mut i = to;
        while i != 0 {
            res = A::binary_operation(res, self.buffer[i].clone());
            i -= lowest_bit(i).unwrap();
        }
        res
    }

    pub fn prefix(&self, to: usize) -> A::T {
        if to == 0 {
            A::identity().get()
        } else {
            self.prefix_inner(to - 1).get()
        }
    }

    pub fn range<R: RangeBounds<usize>>(&self, range: R) -> A::T {
        let (from, to) = util::expand_range_bound(&range, 0, self.len());
        if from == 0 {
            self.prefix(to)
        } else {
            A::binary_operation(self.prefix_inner(to - 1), self.prefix_inner(from - 1).inv()).get()
        }
    }

    pub fn add(&mut self, mut i: usize, value: A::T) {
        if i == 0 {
            self.buffer[0] = A::binary_operation(self.buffer[0].clone(), value.into());
        } else {
            while i < self.len() {
                self.buffer[i] = A::binary_operation(self.buffer[i].clone(), value.clone().into());
                i += lowest_bit(i).unwrap();
            }
        }
    }
}

fn lowest_bit(x: usize) -> Option<usize> {
    if x == 0 {
        None
    } else {
        let s = x.trailing_zeros();
        Some(1 << s)
    }
}
