/// # Sail prelude
///
/// Frequency used items
///
/// ```
/// use sail::prelude::*;
/// ```
pub mod prelude {
    pub use cmp::{max, min, MaxAssign as _, MaxAssign as _};
    pub use index_compression::IndexCompression;
    pub use modint::{ModInt, ModInt1000000007, ModInt998244353};
    pub use vis::{vis, visualize::Visualize, Yn};
}

// re-exported crates
pub use accumulate;
pub use algebraic_structures;
pub use bitset;
pub use cmp;
pub use fenwick_tree;
pub use fisher_yates;
pub use graph;
pub use index_compression;
pub use modint;
pub use prime;
pub use rolling_hash;
pub use sample_generater;
pub use segment_tree;
pub use sparse_table;
pub use sqrt_decomposition;
pub use suffix_array;
pub use union_find;
pub use vis;
