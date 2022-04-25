/// # Sail prelude
///
/// Frequency used items
///
/// ```
/// use sail::prelude::*;
/// ```
pub mod prelude {
    pub use crate::consts::*;
    pub use cmp::{max, min, MaxAssign as _, MaxAssign as _};
    pub use index_compression::IndexCompression;
    pub use io::{
        interactive_input,
        output::util::{polar_question, Yn},
        vis,
    };
    pub use modint::{ModInt, ModInt1000000007, ModInt998244353};
}

pub mod consts {

    pub const MOD_1000000007: u64 = 1000000007;
    pub const MOD_998244353: u64 = 998244353;
    pub const INF: i64 = 2000000000;
    pub const FNI: i64 = -2000000000;
    pub const PI: f64 = std::f64::consts::PI;
    pub const ALPHABET_LARGE: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";
    pub const ALPHABET_SMALL: &str = "abcdefghijklmnopqrstuvwxyz";
    pub const ADJ4: [(i64, i64); 4] = [(1, 0), (0, 1), (-1, 0), (0, -1)];
    pub const ADJ8: [(i64, i64); 8] = [
        (1, 0),
        (1, 1),
        (0, 1),
        (-1, 1),
        (-1, 0),
        (-1, -1),
        (0, -1),
        (1, -1),
    ];
}

// re-exported crates
pub use accumulate;
pub use algebraics;
pub use bitset;
pub use cmp;
pub use fenwick_tree;
pub use fisher_yates;
pub use graph;
pub use index_compression;
pub use io;
pub use modint;
pub use prime;
pub use rolling_hash;
pub use sample_generater;
pub use segment_tree;
pub use sparse_table;
pub use sqrt_decomposition;
pub use suffix_array;
pub use union_find;
