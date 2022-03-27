//! This solution is created by SaiYS, @awpsyrhy(Twitter)
#![allow(unused_imports)]

// #[fastout]
fn main() {
    input! {}
    todo!("You can solve it!")
}

use sail::*;

use im_rc::{ordmap, ordset, OrdMap, OrdSet};
use itertools::{iproduct, izip, Itertools as _};
use itertools_num::ItertoolsNum as _;
use maplit::{btreemap, btreeset, convert_args, hashmap, hashset};
use num::{
    bigint::{BigInt, BigUint, ToBigInt, ToBigUint},
    complex::Complex64,
    integer::{binomial, gcd, gcd_lcm, lcm, multinomial, Integer},
    traits::{abs, abs_sub, Bounded, One, Pow, Saturating, Zero},
};
use proconio::{
    fastout, input,
    marker::{Bytes, Chars, Isize1, Usize1},
    source::{auto::AutoSource, line::LineSource, once::OnceSource},
};
use rand::{random, rngs::SmallRng, Rng, SeedableRng};
use std::{
    cmp::{max, min},
    collections::{BTreeMap, BTreeSet, BinaryHeap, VecDeque},
    convert::{From, Into},
    f64::consts::PI,
    str::FromStr,
    string::ToString,
    usize::MAX,
};

pub type HashSet<T> = rustc_hash::FxHashSet<T>;
pub type HashMap<K, V> = rustc_hash::FxHashMap<K, V>;

pub const MOD_1000000007: usize = 1000000007;
pub const MOD_998244353: usize = 998244353;
pub const INF: usize = 2000000000;
pub const FNI: i64 = -2000000000;
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
