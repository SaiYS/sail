use std::{
    cmp::{max, min},
    ops::RangeBounds,
};

pub use algebraic_structures::{monoid::Monoid, semigroup::SemiGroup};

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
                    let from = i * block_size;
                    let to = min(from + block_size, len);
                    let mut sum = M::identity();
                    for j in from..to {
                        sum.binary_operation_assign(original[j].clone());
                    }
                    sum
                })
                .collect(),
            original,
        }
    }

    pub fn len(&self) -> usize {
        self.len
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
        self.block_sum[i / self.block_size] = M::fold(self.nth_block(i / self.block_size)).unwrap();
    }

    /// Returns one value at index i
    pub fn get(&mut self, i: usize) -> M::T {
        self.original[i].clone().get()
    }

    /// Returns a folded value in `range`
    pub fn get_range<R: RangeBounds<usize>>(&mut self, range: R) -> M::T {
        let (from, to) = util::expand_range_bound(&range, 0, self.len());
        let mut res = M::identity();
        for i in 0..self.block_len() {
            let (l, r) = self.block_range(i);
            if from <= l && r <= to {
                res.binary_operation_assign(self.block_sum[i].clone());
            } else if r <= from || to <= l {
                continue;
            } else {
                res.binary_operation_assign(
                    M::fold(&self.original[max(from, l)..min(to, r)]).unwrap(),
                );
            }
        }

        res.get()
    }
}

#[cfg(test)]
mod tests {
    use super::Decomposition;
    use algebraic_structures::{monoid::MMin, semigroup::SemiGroup};
    use rand::{thread_rng, Rng};
    use std::{iter::repeat_with, mem::swap};

    fn verify_sqrt_decomposition() {
        let mut rng = thread_rng();
        let n = 2000;
        let a = repeat_with(|| rng.gen_range(0..n))
            .take(n)
            .collect::<Vec<usize>>();
        let mut sd = Decomposition::<MMin<usize>>::from(a);

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
                    MMin::fold(&sd.original[from..to]).unwrap().get()
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
