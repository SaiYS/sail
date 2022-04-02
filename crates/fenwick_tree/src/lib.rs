pub mod abelian_group;
use std::ops::{Index, IndexMut, RangeBounds};

use abelian_group::AbelianGroup;

#[derive(Debug, Clone)]
pub struct FenwickTree<T> {
    len: usize,
    buffer: Vec<T>,
}

impl<A> Index<usize> for FenwickTree<A> {
    type Output = A;

    fn index(&self, index: usize) -> &Self::Output {
        &self.buffer[index]
    }
}

impl<A> IndexMut<usize> for FenwickTree<A> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        self.buffer.index_mut(index)
    }
}

impl<A: AbelianGroup> FenwickTree<A> {
    pub fn new(n: usize) -> Self {
        Self {
            len: n + 1,
            buffer: vec![A::identity(); n + 1],
        }
    }

    pub fn len(&self) -> usize {
        self.len
    }
}

impl<A: AbelianGroup> FenwickTree<A> {
    fn prefix_sum_inner(&self, to: usize) -> A {
        if to >= self.len() {
            panic!("panicked at 'index out of bounds'");
        }
        let mut res = self[0].clone();
        let mut i = to;
        while i != 0 {
            res = A::add(res, self[i].clone());
            i -= lowest_bit(i).unwrap();
        }
        res
    }

    pub fn prefix_sum(&self, to: usize) -> A::T {
        if to == 0 {
            A::identity().get()
        } else {
            self.prefix_sum_inner(to - 1).get()
        }
    }

    pub fn range_sum<R: RangeBounds<usize>>(&self, range: R) -> A::T {
        let (from, to) = util::expand_range_bound(&range, 0, self.len());
        debug_assert!(from < to);

        if from == 0 {
            self.prefix_sum(to)
        } else {
            A::add(self.prefix_sum_inner(to), self.prefix_sum_inner(from).inv()).get()
        }
    }

    pub fn add(&mut self, mut i: usize, value: A::T) {
        if i >= self.len() {
            panic!("panicked at 'index out of bounds'");
        }

        let value = A::set(value);

        if i == 0 {
            self.buffer[0] = A::add(self[0].clone(), value.clone());
        } else {
            while i < self.len() {
                self.buffer[i] = A::add(self[i].clone(), value.clone());
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
