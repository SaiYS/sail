use std::ops::RangeBounds;

use rand::{Rng, SeedableRng};

pub fn expand_range_bound<R: RangeBounds<usize>>(r: &R, min: usize, max: usize) -> (usize, usize) {
    let from = match r.start_bound() {
        std::ops::Bound::Included(start) => *start,
        std::ops::Bound::Excluded(_) => unimplemented!(),
        std::ops::Bound::Unbounded => min,
    };
    let to = match r.end_bound() {
        std::ops::Bound::Included(end) => *end + 1,
        std::ops::Bound::Excluded(end) => *end,
        std::ops::Bound::Unbounded => max,
    };

    (from, to)
}

pub fn shuffle<T>(mut v: Vec<T>) -> Vec<T> {
    let n = v.len();
    let mut rng = rand::rngs::SmallRng::from_entropy();
    for i in (1..n).rev() {
        let j = rng.gen_range(0, i + 1);
        v.swap(i, j);
    }
    v
}

pub trait Shufflable {
    fn shuffle(self) -> Self;
}

impl<T> Shufflable for Vec<T> {
    fn shuffle(self) -> Self {
        shuffle(self)
    }
}

#[test]
fn s() {
    let v = (0..10).collect::<Vec<_>>();
    dbg!(shuffle(v));
}
