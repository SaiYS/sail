pub use algebraics::{
    abstract_type::Monoid,
    structure::{Additive, Gcd, Lcm, Max, Min, Multiplicative},
};

// pub mod delayed_segment_tree;
// pub mod dual_segment_tree;
pub mod segment_tree;
pub use segment_tree::SegmentTree;

pub type RangeMin<T> = SegmentTree<Min<T>>;
pub type RangeMax<T> = SegmentTree<Max<T>>;
pub type RangeSum<T> = SegmentTree<Additive<T>>;
