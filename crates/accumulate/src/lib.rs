use num_traits::Zero;
use std::ops::{Add, Index, RangeBounds, Sub};
use util::expand_range_bound;

pub mod imos;

pub struct Accumulation<T> {
    len: usize,
    buffer: Vec<T>,
}

impl<T: Clone + Add + Zero> From<Vec<T>> for Accumulation<T> {
    fn from(v: Vec<T>) -> Self {
        let len = v.len() + 1;
        let mut buffer = vec![T::zero()];
        v.into_iter().fold(T::zero(), |mut acc, x| {
            acc = acc + x;
            buffer.push(acc.clone());
            acc
        });

        Self { len, buffer }
    }
}

impl<T: Clone + Add + Zero> From<&[T]> for Accumulation<T> {
    fn from(v: &[T]) -> Self {
        let len = v.len() + 1;
        let mut buffer = vec![T::zero()];
        v.iter().fold(T::zero(), |mut acc, x| {
            acc = acc + x.clone();
            buffer.push(acc.clone());
            acc
        });

        Self { len, buffer }
    }
}

impl<T> Index<usize> for Accumulation<T> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        self.buffer.index(index)
    }
}

impl<T: Clone + Add<Output = T> + Sub<Output = T>> Accumulation<T> {
    /// Returns the length of original array,
    ///
    /// Indiced 0, 1, .. , self.len() - 1, are availble
    pub fn len(&self) -> usize {
        self.len
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn accumulation(&self) -> &[T] {
        &self.buffer
    }

    /// Returns sum of values in `range`
    ///
    /// Complexity: `O(1)`
    pub fn range_sum<R: RangeBounds<usize>>(&self, range: R) -> T {
        let (from, to) = expand_range_bound(&range, 0, self.len());
        self[to].clone() - self[from].clone()
    }
}

#[test]
fn accumulation_test() {
    let v = vec![1, 2, 3];
    let a = Accumulation::from(v);
    assert_eq!(a.range_sum(0..1), 1);
    assert_eq!(a.range_sum(0..2), 3);
    assert_eq!(a.range_sum(0..3), 6);
    assert_eq!(a.range_sum(1..3), 5);
    assert_eq!(a.range_sum(2..3), 3);
}
