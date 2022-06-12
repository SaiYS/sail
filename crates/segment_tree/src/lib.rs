pub use algebraics::{
    abstruct::Monoid,
    structure::{Additive, Gcd, Lcm, Max, Min, Multiplicative},
};

// pub mod delayed_segment_tree;
// pub mod dual_segment_tree;
pub mod segment_tree;
pub use segment_tree::SegmentTree;

pub type RangeMinQ<T> = SegmentTree<Min<T>>;
pub type RangeSumQ<T> = SegmentTree<Additive<T>>;
