use algebraics::{abstract_type::SemiGroup, property::Idempotent};
use itertools::Itertools;
use std::ops::RangeBounds;
use util::expand_range_bound;

pub use algebraics::structure::{Gcd, Lcm, Max, Min};

#[derive(Debug, Clone)]
pub struct SparseTable<S: SemiGroup + Idempotent<S::I>> {
    len: usize,
    buffer: Vec<Vec<S::I>>,
}

impl<S: SemiGroup + Idempotent<S::I>> From<Vec<S::I>> for SparseTable<S> {
    fn from(v: Vec<S::I>) -> Self {
        Self::new(v)
    }
}

impl<S: SemiGroup + Idempotent<S::I>> SparseTable<S> {
    pub fn new(v: Vec<S::I>) -> Self {
        let len = v.len();
        let rank = len.next_power_of_two().trailing_zeros() as usize;
        let mut buffer: Vec<Vec<S::I>> = vec![Vec::new(); rank];
        buffer[0] = v.into_iter().collect_vec();
        for (height, width) in (1..rank).map(|x| (x, 1 << x)) {
            buffer[height] = (0..=len - width)
                .map(|i| {
                    S::operate(
                        buffer[height - 1][i].clone(),
                        buffer[height - 1][i + width / 2].clone(),
                    )
                })
                .collect_vec();
        }
        Self { len, buffer }
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn range<R: RangeBounds<usize>>(&self, range: R) -> S::I {
        let (from, to) = expand_range_bound(&range, 0, self.len());

        if to - from == 1 {
            self.buffer[0][from].clone()
        } else {
            let h = (to - from).next_power_of_two().trailing_zeros() as usize - 1;
            let w = to - (1 << h);
            S::operate(self.buffer[h][from].clone(), self.buffer[h][w].clone())
        }
    }
}

#[cfg(test)]
mod tests {
    use algebraics::structure::Min;
    use itertools::Itertools;
    use rand::{thread_rng, Rng};

    use crate::SparseTable;

    #[test]
    fn run_verify_sparse_table() {
        for _ in 0..100 {
            verify_sparse_table();
        }
    }

    fn verify_sparse_table() {
        let mut rng = thread_rng();

        let n = 1000;
        let a: Vec<u32> = (0..n).map(|_| rng.gen_range(0..1000)).collect_vec();
        let st = SparseTable::<Min<u32>>::new(a.clone());

        for _ in 0..1000 {
            let mut from = rng.gen_range(0..n);
            let mut to = rng.gen_range(0..n);

            if from > to {
                std::mem::swap(&mut from, &mut to);
            }

            assert_eq!(st.range(from..=to), *a[from..=to].iter().min().unwrap());
        }
    }
}
