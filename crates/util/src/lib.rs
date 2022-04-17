use std::ops::RangeBounds;

/// Expand range expressions, like, `from..to`, `from..=to`
///
/// returns a tuple of two integer `[from, to)`
///
/// when start_bound is Unbounded, uses min
/// when end_bound is Unbounded, uses max
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
