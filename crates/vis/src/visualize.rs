pub trait Visualize {
    fn visualize(&self, split: &str) -> String;

    fn continuous(&self) -> String {
        self.visualize("")
    }

    fn spaces(&self) -> String {
        self.visualize(" ")
    }

    fn lines(&self) -> String {
        self.visualize("\n")
    }
}

macro_rules! impl_visualize_for_primitives {
($($t:ty),+) => {
    $(
        impl Visualize for $t {
            fn visualize(&self, _split: &str) -> String {
                format!("{}", self)
            }
        }
    )+
};
}

impl_visualize_for_primitives! {
    usize, u8, u16, u32, u64, u128,
    isize, i8, i16, i32, i64, i128,
    f32, f64,
    String, &str, char
}

use itertools::Itertools as _;
use std::{
    collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque},
    fmt::Display,
};

impl<T: Display, U: Display> Visualize for (T, U) {
    fn visualize(&self, _split: &str) -> String {
        format!("{} {}", self.0, self.1)
    }
}

impl<T: Display, U: Display, V: Display> Visualize for (T, U, V) {
    fn visualize(&self, _split: &str) -> String {
        format!("{} {} {}", self.0, self.1, self.2)
    }
}

impl<T: Display, U: Display, V: Display, W: Display> Visualize for (T, U, V, W) {
    fn visualize(&self, _split: &str) -> String {
        format!("{} {} {} {}", self.0, self.1, self.2, self.3)
    }
}

impl<T: Display, U: Display, V: Display, W: Display, X: Display> Visualize for (T, U, V, W, X) {
    fn visualize(&self, _split: &str) -> String {
        format!("{} {} {} {} {}", self.0, self.1, self.2, self.3, self.4)
    }
}

impl<T: Display> Visualize for [T] {
    fn visualize(&self, split: &str) -> String {
        self.iter().join(split)
    }
}

impl<T: Display> Visualize for &[T] {
    fn visualize(&self, split: &str) -> String {
        self.iter().join(split)
    }
}

impl<T: Display> Visualize for VecDeque<T> {
    fn visualize(&self, split: &str) -> String {
        self.iter().join(split)
    }
}

impl<T: Display> Visualize for BinaryHeap<T> {
    fn visualize(&self, split: &str) -> String {
        self.iter().join(split)
    }
}

impl<T: Display> Visualize for HashSet<T> {
    fn visualize(&self, split: &str) -> String {
        self.iter().join(split)
    }
}

impl<K: Display, V: Display> Visualize for HashMap<K, V> {
    fn visualize(&self, split: &str) -> String {
        self.iter().map(|(k, v)| format!("{} {}", k, v)).join(split)
    }
}

impl<T: Display> Visualize for BTreeSet<T> {
    fn visualize(&self, split: &str) -> String {
        self.iter().join(split)
    }
}

impl<K: Display, V: Display> Visualize for BTreeMap<K, V> {
    fn visualize(&self, split: &str) -> String {
        self.iter().map(|(k, v)| format!("{} {}", k, v)).join(split)
    }
}
