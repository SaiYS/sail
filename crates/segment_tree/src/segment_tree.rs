use algebraics::abstruct::Monoid;
use itertools::Itertools;
use std::{
    fmt::{Debug, Display},
    ops::RangeBounds,
};

/// Generic segment-tree
#[derive(Clone)]
pub struct SegmentTree<M: Monoid> {
    len: usize,
    capacity: usize,
    size: usize,
    height: usize,
    buffer: Vec<M::I>,
}

impl<M> Debug for SegmentTree<M>
where
    M: Monoid,
    M::I: Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = (0..self.height())
            .map(|x| {
                ((1 << x) - 1..)
                    .take(1 << x)
                    .map(|x| &self.buffer[x])
                    .join(" ")
            })
            .join("\n");
        write!(f, "\n{}", s)
    }
}

impl<M: Monoid> From<Vec<M::I>> for SegmentTree<M> {
    /// Complexity: O(n)
    fn from(v: Vec<M::I>) -> Self {
        let len = v.len();
        let capacity = len.next_power_of_two();
        let height = capacity.trailing_zeros() as usize + 1;
        let size = capacity * 2 - 1;
        let mut buffer = vec![<M as Monoid>::identity(); size];

        for (i, e) in v.into_iter().enumerate() {
            buffer[size / 2 + i] = e;
        }

        for i in (0..capacity - 1).rev() {
            buffer[i] = M::operate(buffer[i * 2 + 1].clone(), buffer[i * 2 + 2].clone());
        }

        Self {
            len,
            capacity,
            size,
            height,
            buffer,
        }
    }
}

impl<M: Monoid> From<&[M::I]> for SegmentTree<M> {
    fn from(v: &[M::I]) -> Self {
        let len = v.len();
        let capacity = len.next_power_of_two();
        let height = capacity.trailing_zeros() as usize + 1;
        let size = capacity * 2 - 1;
        let mut buffer = vec![<M as Monoid>::identity(); size];

        for (i, e) in v.iter().enumerate() {
            buffer[size / 2 + i] = e.clone();
        }

        for i in (0..capacity - 1).rev() {
            buffer[i] = M::operate(buffer[i * 2 + 1].clone(), buffer[i * 2 + 2].clone());
        }

        Self {
            len,
            capacity,
            size,
            height,
            buffer,
        }
    }
}

impl<M: Monoid> SegmentTree<M> {
    /// Returns the size of its buffer
    pub fn size(&self) -> usize {
        self.size
    }

    /// Returns the length of the original array, NOT size of its buffer
    pub fn len(&self) -> usize {
        self.len
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Returns the height of the tree
    pub fn height(&self) -> usize {
        self.height
    }

    /// Create a new empty SegmentTree with given length
    pub fn new(len: usize) -> Self {
        let capacity = len.next_power_of_two();
        let height = capacity.trailing_zeros() as usize + 1;
        let size = capacity * 2 - 1;
        Self {
            len,
            capacity,
            size,
            height,
            buffer: vec![<M as Monoid>::identity(); size],
        }
    }

    /// Returns ref of original array sliced from its buffer
    pub fn raw_leaves(&self) -> &[M::I] {
        &self.buffer[self.capacity - 1..self.size]
    }

    /// Returns a value of i-th leaf
    ///
    /// Complexity: O(1)
    pub fn get(&self, i: usize) -> M::I {
        self.buffer[self.capacity - 1 + i].clone()
    }

    /// Returns a folded value of leaves in `range`
    ///
    /// Complexity: O(log n)
    pub fn range<R: RangeBounds<usize>>(&self, range: R) -> M::I {
        let (from, to) = util::expand_range_bound(&range, 0, self.len());
        debug_assert!(from < to);

        let mut from = from + self.capacity - 1;
        let mut to = to + self.capacity - 1;

        let mut ls = <M as Monoid>::identity();
        let mut rs = <M as Monoid>::identity();

        while from < to {
            if from & 1 == 0 {
                M::operate_assign(&mut ls, self.buffer[from].clone());
                from += 1;
            }
            if to & 1 == 0 {
                to -= 1;
                M::operate_assign(&mut rs, self.buffer[to].clone());
            }
            from = (from - 1) >> 1;
            to = (to - 1) >> 1;
        }

        M::operate(ls, rs)
    }

    /// Returns a folded value of all leaves
    ///
    /// Complexity: O(1)
    /// This can be more efficient than calling `self.get_range(..)`
    pub fn all(&self) -> M::I {
        self.buffer[0].clone()
    }

    /// Update one value at index `i` with `new_value`
    ///
    /// Complexity: O(log n)
    pub fn update(&mut self, i: usize, new_value: M::I) {
        let mut cur = self.capacity - 1 + i;
        self.buffer[cur] = new_value;
        while cur != 0 {
            cur = (cur - 1) >> 1;
            self.buffer[cur] = M::operate(
                self.buffer[cur * 2 + 1].clone(),
                self.buffer[cur * 2 + 2].clone(),
            )
        }
    }
}

#[cfg(test)]
mod tests {
    use super::SegmentTree;
    use algebraics::{abstruct::Monoid, structure::Min};
    use itertools::Itertools;
    use rand::Rng;

    fn verify() {
        let mut rng = rand::thread_rng();

        let n = 1000usize;
        let a = (0..n).map(|_| rng.gen_range(0..n)).collect_vec();

        let mut st = SegmentTree::<Min<usize>>::from(a);

        for _ in 0..n {
            if rng.gen_bool(0.8) {
                // query
                let mut from = rng.gen_range(0..n);
                let mut to = rng.gen_range(0..n);

                if from > to {
                    std::mem::swap(&mut from, &mut to);
                }

                assert_eq!(
                    st.range(from..=to),
                    Min::fold_right(&st.raw_leaves()[from..=to])
                );
            } else {
                // update
                let i = rng.gen_range(0..n);
                let value = rng.gen_range(0..n);
                st.update(i, value);
            }
        }
    }

    #[test]
    fn run_varify() {
        for _ in 0..100 {
            verify();
        }
    }
}
