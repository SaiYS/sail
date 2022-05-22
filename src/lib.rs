/// # Sail prelude
///
/// Frequency used items
///
/// ```
/// use sail::prelude::*;
/// ```
pub mod prelude {
    pub use crate::consts::*;
    pub use cmp::{max, min, MaxAssign as _, MinAssign as _};
    pub use counter::Counter;
    pub use index_compression::IndexCompression;
    pub use io::{
        dvis, interactive_input,
        output::util::{polar_question, Yn},
        proconio::{
            fastout, input,
            marker::{Bytes, Chars, Isize1, Usize1},
            source::{auto::AutoSource, line::LineSource, once::OnceSource},
        },
        vis,
    };
    pub use modint::{ModInt1000000007, ModInt998244353, StaticModInt as ModInt};
}

pub mod consts;

// re-exported crates
pub use accumulate;
pub use algebraics;
pub use bitset;
pub use cmp;
pub use counter;
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
pub use timer;
