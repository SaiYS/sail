use std::{collections::HashMap, hash::Hash};

pub trait ItertoolsSail: Iterator {
    fn counts(self) -> HashMap<Self::Item, usize>
    where
        Self: Sized,
        Self::Item: Eq + Hash,
    {
        let mut counts = HashMap::new();
        for ele in self {
            *counts.entry(ele).or_insert(0) += 1;
        }
        counts
    }
}

impl<T: ?Sized> ItertoolsSail for T where T: Iterator {}
