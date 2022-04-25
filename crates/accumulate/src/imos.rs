use num_traits::Zero;
use std::ops::{Add, Neg, RangeBounds};
use util::expand_range_bound;

/// Imos Algorythm
///
/// See https://imoz.jp/algorithms/imos_method.html
#[derive(Debug, Clone)]
pub struct Imos<T> {
    _lock: bool,
    len: usize,
    diff: Vec<T>,
    acc: Vec<T>,
}

impl<T: Clone + Zero + Add<Output = T> + Neg<Output = T>> Imos<T> {
    pub fn new(len: usize) -> Self {
        Self {
            _lock: true,
            len,
            diff: vec![T::zero(); len + 1],
            acc: vec![T::zero(); len],
        }
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    fn build(&mut self) {
        if !self._lock {
            let mut acc = T::zero();
            for i in 0..self.len() {
                acc = acc.clone() + self.diff[i].clone();
                self.acc[i] = acc.clone();
            }
            self._lock = true
        }
    }

    pub fn range_add<R: RangeBounds<usize>>(&mut self, value: T, range: R) {
        let (from, to) = expand_range_bound(&range, 0, self.len);

        self.diff[from] = self.diff[from].clone() + value.clone();
        self.diff[to] = self.diff[to].clone() + (-value);
        self._lock = false;
    }

    pub fn accumulation(&mut self) -> &[T] {
        self.build();
        &self.acc
    }
}

#[test]
fn debug() {
    let mut imos = Imos::<i32>::new(5);
    dbg!(&imos);
    imos.range_add(1, ..);
    dbg!(&imos);
    imos.range_add(1, 1..4);
    dbg!(&imos);
    imos.range_add(-5, 2..3);
    imos.build();
    dbg!(&imos);
}
