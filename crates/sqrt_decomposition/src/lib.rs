use algebraics::property::Identity;
use std::{
    cmp::{max, min},
    ops::RangeBounds,
};

pub use algebraics::{
    abstruct::Monoid,
    structure::{Additive, Gcd, Lcm, Max, Min, Multiplicative},
};

#[derive(Debug, Clone)]
pub struct Decomposition<M: Monoid> {
    len: usize,
    block_size: usize,
    original: Vec<M>,
    block_sum: Vec<M>,
}

impl<M: Monoid> From<Vec<M::T>> for Decomposition<M> {
    fn from(v: Vec<M::T>) -> Self {
        let sqrt = (v.len() as f64).sqrt().round() as usize;
        Self::new(v, sqrt)
    }
}

impl<M: Monoid> Decomposition<M> {
    /// Create a new `Decomposition`
    pub fn new(v: Vec<M::T>, block_size: usize) -> Self {
        let len = v.len();
        let original: Vec<M> = v.into_iter().map(|x| x.into()).collect();

        Self {
            len,
            block_size,
            block_sum: (0..)
                .take_while(|&x| x * block_size < len)
                .map(|i| {
                    original
                        .iter()
                        .take(min(i * block_size + block_size, len))
                        .skip(i * block_size)
                        .fold(Identity::identity(), |mut sum: M, x| {
                            sum.operate_assign(x.clone());
                            sum
                        })
                })
                .collect(),
            original,
        }
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    fn block_len(&self) -> usize {
        self.block_sum.len()
    }

    fn block_range(&self, n: usize) -> (usize, usize) {
        (
            self.block_size * n,
            min(self.block_size * n + self.block_size, self.len),
        )
    }

    fn nth_block(&self, n: usize) -> &[M] {
        let (from, to) = self.block_range(n);
        &self.original[from..to]
    }

    /// Update one value at index i into new_value
    pub fn update(&mut self, i: usize, new_value: M::T) {
        self.original[i] = new_value.into();
        self.block_sum[i / self.block_size] = M::fold_right(self.nth_block(i / self.block_size));
    }

    /// Returns one value at index i
    pub fn get(&mut self, i: usize) -> M::T {
        self.original[i].clone().get()
    }

    /// Returns a folded value in `range`
    pub fn get_range<R: RangeBounds<usize>>(&mut self, range: R) -> M::T {
        let (from, to) = util::expand_range_bound(&range, 0, self.len());
        let mut res: M = Identity::identity();
        for i in 0..self.block_len() {
            let (l, r) = self.block_range(i);
            if from <= l && r <= to {
                res.operate_assign(self.block_sum[i].clone());
            } else if r <= from || to <= l {
                continue;
            } else {
                res.operate_assign(M::fold_right(&self.original[max(from, l)..min(to, r)]));
            }
        }

        res.get()
    }
}

#[cfg(test)]
mod tests {
    use super::Decomposition;
    use algebraics::{abstruct::Monoid, structure::Max};
    use rand::{thread_rng, Rng};
    use std::{iter::repeat_with, mem::swap};

    fn verify_sqrt_decomposition() {
        let mut rng = thread_rng();
        let n = 1000;
        let a = repeat_with(|| rng.gen_range(0..n))
            .take(n)
            .collect::<Vec<usize>>();
        let mut sd = Decomposition::<Max<usize>>::from(a);

        for _ in 0..n {
            if rng.gen_bool(0.8) {
                let mut from = rng.gen_range(0..n);
                let mut to = rng.gen_range(0..n);
                if from == to {
                    continue;
                }
                if to < from {
                    swap(&mut from, &mut to);
                }
                assert_eq!(
                    sd.get_range(from..to),
                    Max::fold_right(&sd.original[from..to]).get()
                );
            } else {
                let x = rng.gen_range(0..n);
                let new_value = rng.gen_range(0..n);
                sd.update(x, new_value);
            }
        }
    }

    #[test]
    fn run_verification() {
        for _ in 0..100 {
            verify_sqrt_decomposition();
        }
    }
}
