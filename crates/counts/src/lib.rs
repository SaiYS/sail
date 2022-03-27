use std::{collections::HashMap, hash::Hash};

pub fn counts<'a, T: Eq + Hash>(v: &'a [T]) -> HashMap<&'a T, usize> {
    let mut res = HashMap::new();
    v.iter().for_each(|x| {
        *res.entry(x).or_insert(0) += 1;
    });
    res
}

#[test]
fn test_count() {
    let v = vec![1, 2, 2, 3, 3, 3];
    let counter = counts(&v);
    assert_eq!(counter, maplit::hashmap! {&1 => 1, &2 => 2, &3 => 3});
}
