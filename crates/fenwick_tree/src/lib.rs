use algebraic_structures::group::Group;
use std::ops::{Index, IndexMut, RangeBounds};

#[derive(Debug, Clone)]
pub struct FenwickTree<T> {
    len: usize,
    buffer: Vec<T>,
}

impl<A> Index<usize> for FenwickTree<A> {
    type Output = A;

    fn index(&self, index: usize) -> &Self::Output {
        self.buffer.index(index)
    }
}

impl<A> IndexMut<usize> for FenwickTree<A> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        self.buffer.index_mut(index)
    }
}

impl<A: Group> FenwickTree<A> {
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

impl<A: Group> FenwickTree<A> {
    fn prefix_sum_inner(&self, to: usize) -> A {
        let mut res = self[0].clone();
        let mut i = to;
        while i != 0 {
            res = A::binary_operation(res, self[i].clone());
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
        if from == 0 {
            self.prefix_sum(to)
        } else {
            A::binary_operation(self.prefix_sum_inner(to), self.prefix_sum_inner(from).inv()).get()
        }
    }

    pub fn add(&mut self, mut i: usize, value: A::T) {
        if i == 0 {
            self[0] = A::binary_operation(self[0].clone(), value.into());
        } else {
            while i < self.len() {
                self[i] = A::binary_operation(self[i].clone(), value.clone().into());
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
