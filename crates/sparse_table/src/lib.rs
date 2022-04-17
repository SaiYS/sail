pub use algebraic_structures::semigroup::SemiGroup;
use itertools::Itertools;
use std::ops::RangeBounds;
use util::expand_range_bound;

#[derive(Debug, Clone)]
pub struct SparseTable<S: SemiGroup> {
    len: usize,
    buffer: Vec<Vec<S>>,
}

impl<S: SemiGroup> From<Vec<S::T>> for SparseTable<S> {
    fn from(v: Vec<S::T>) -> Self {
        Self::new(v)
    }
}

impl<S: SemiGroup> SparseTable<S> {
    pub fn new(v: Vec<S::T>) -> Self {
        let len = v.len();
        let rank = len.next_power_of_two().trailing_zeros() as usize;
        let mut buffer: Vec<Vec<S>> = vec![Vec::new(); rank];
        buffer[0] = v.into_iter().map(|x| x.into()).collect_vec();
        for (height, width) in (1..rank).map(|x| (x, 1 << x)) {
            buffer[height] = (0..=len - width)
                .map(|i| {
                    S::binary_operation(
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

    pub fn range<R: RangeBounds<usize>>(&self, range: R) -> S::T {
        let (from, to) = expand_range_bound(&range, 0, self.len());

        if to - from == 1 {
            self.buffer[0][from].clone()
        } else {
            let h = (to - from).next_power_of_two().trailing_zeros() as usize - 1;
            // let h = self.rank() - (BITS - ((from) ^ (to - 1)).leading_zeros() - 1) as usize;
            let w = to - (1 << h);
            S::binary_operation(self.buffer[h][from].clone(), self.buffer[h][w].clone())
        }
        .get()
    }
}
