use itertools::Itertools;
use monoid::Monoid;
use std::fmt::{Debug, Display};
use std::ops::RangeBounds;

pub mod monoid;

#[derive(Debug, Clone)]
pub struct SegmentTree<M> {
    n: usize,
    buffer: Vec<M>,
}

impl<M> SegmentTree<M> {
    pub fn size(&self) -> usize {
        self.buffer.len()
    }

    pub fn leaf_len(&self) -> usize {
        self.n.next_power_of_two()
    }

    pub fn rank(&self) -> usize {
        self.leaf_len().next_power_of_two().trailing_zeros() as usize + 1
    }

    fn leaf_idx(&self, i: usize) -> usize {
        self.leaf_len() + i - 1
    }
}

impl<M> SegmentTree<M>
where
    M: Monoid + Clone,
    M::T: Display,
{
    pub fn dbg_tree(&self) {
        let mut cur = 0;
        for rank in 0..self.rank() {
            let l = 1 << rank;
            let s = self.buffer[cur..(cur + l)]
                .iter()
                .map(|x| x.clone().get())
                .join(" ");
            println!("{}", s);
            cur += l;
        }
    }
}

impl<M> SegmentTree<M>
where
    M: Monoid + Clone,
    M::T: Clone,
{
    pub fn new(n: usize) -> Self {
        Self {
            n,
            buffer: vec![M::identity(); n.next_power_of_two() * 2 - 1],
        }
    }

    fn update_inner(&mut self, i: usize, value: M::T) {
        self.buffer[i] = M::set(value.clone());
        if i > 0 {
            self.update_inner(
                (i - 1) >> 1,
                M::binary_operation(
                    self.buffer[i].clone(),
                    self.buffer[if i & 1 != 0 { i + 1 } else { i - 1 }].clone(),
                )
                .get(),
            );
        }
    }

    pub fn update(&mut self, i: usize, value: M::T) {
        let cur = self.leaf_idx(i);
        self.update_inner(cur, value);
    }

    fn range_inner(&self, from: usize, to: usize, l: usize, r: usize, k: usize) -> M {
        if l >= to || r <= from {
            M::identity()
        } else if from <= l && r <= to {
            self.buffer[k].clone()
        } else {
            let m = (l + r) / 2;
            M::binary_operation(
                self.range_inner(from, to, l, m, k * 2 + 1),
                self.range_inner(from, to, m, r, k * 2 + 2),
            )
        }
    }

    pub fn range<R: RangeBounds<usize>>(&self, range: R) -> M::T {
        let from = match range.start_bound() {
            std::ops::Bound::Included(start) => *start,
            std::ops::Bound::Excluded(_) => unimplemented!(),
            std::ops::Bound::Unbounded => 0,
        };
        let to = match range.end_bound() {
            std::ops::Bound::Included(end) => *end + 1,
            std::ops::Bound::Excluded(end) => *end,
            std::ops::Bound::Unbounded => self.leaf_len(),
        };

        self.range_inner(from, to, 0, self.leaf_len(), 0).get()
    }
}
