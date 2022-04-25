pub use algebraics::abstruct::AbelianGroup;

use algebraics::{property::Identity, structure::Additive};
use std::ops::RangeBounds;

#[derive(Debug, Clone)]
pub struct FenwickTree<A: AbelianGroup> {
    len: usize,
    buffer: Vec<A>,
}

pub type AdditiveFenwickTree = FenwickTree<Additive<i64>>;

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
            buffer: vec![Identity::identity(); len + 1],
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
            res.operate_assign(self.buffer[i].clone());
            i -= lowest_bit(i).unwrap();
        }
        res
    }

    pub fn prefix(&self, to: usize) -> A::T {
        if to == 0 {
            <A as Identity>::identity().get()
        } else {
            self.prefix_inner(to - 1).get()
        }
    }

    pub fn range<R: RangeBounds<usize>>(&self, range: R) -> A::T {
        let (from, to) = util::expand_range_bound(&range, 0, self.len());
        if from == 0 {
            self.prefix(to)
        } else {
            self.prefix_inner(to - 1)
                .operate(self.prefix_inner(from - 1).inverse())
                .get()
        }
    }

    pub fn add(&mut self, mut i: usize, value: A::T) {
        if i == 0 {
            self.buffer[0] = self.buffer[0].clone().operate(value.into());
        } else {
            while i < self.len() {
                self.buffer[i] = self.buffer[i].clone().operate(value.clone().into());
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

#[cfg(test)]
mod tests {
    use crate::FenwickTree;
    use algebraics::{abstruct::AbelianGroup, structure::Additive, Operation};
    use rand::Rng;

    #[test]
    fn run_verify_fenwick_tree() {
        for _ in 0..100 {
            verify_fenwick_tree();
        }
    }

    fn verify_fenwick_tree() {
        let mut rng = rand::thread_rng();

        let n = 1000;

        let mut raw: Vec<Additive<i64>> = vec![0.into(); n];
        let mut ft = FenwickTree::<algebraics::structure::Additive<i64>>::new(n);

        for _ in 0..n {
            if rng.gen_bool(0.8) {
                let mut l = rng.gen_range(0..n);
                let mut r = rng.gen_range(0..n);
                if l > r {
                    std::mem::swap(&mut l, &mut r);
                }

                assert_eq!(ft.range(l..=r), AbelianGroup::fold(&raw[l..=r]).get())
            } else {
                let i = rng.gen_range(0..n);
                let value = rng.gen_range(-100..=100);
                ft.add(i, value);
                raw[i].operate_assign(value.into());
            }
        }
    }
}
