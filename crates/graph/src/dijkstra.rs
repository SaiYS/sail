use itertools::Itertools;
use num_traits::Unsigned;
use std::{cmp::Reverse, collections::BinaryHeap};

pub fn dijkstra<W: Copy + Ord + Unsigned>(g: &[Vec<(usize, W)>], start: usize) -> Vec<Option<W>> {
    let mut dist = vec![None; g.len()];
    dist[start] = Some(W::zero());

    let mut heap = BinaryHeap::new();
    heap.push((Reverse(Some(W::zero())), start));

    while let Some((Reverse(Some(d)), cur)) = heap.pop() {
        for &(next, weight) in g[cur]
            .iter()
            .filter(|&&(x, w)| dist[x].is_none() || dist[x].unwrap() > d + w)
            .collect_vec()
        {
            dist[next] = Some(d + weight);
            heap.push((Reverse(Some(d + weight)), next));
        }
    }

    dist
}

pub fn dijkstra_with_path_hint<W: Copy + Ord + Unsigned>(
    g: &[Vec<(usize, W)>],
    start: usize,
) -> (Vec<Option<W>>, Vec<Option<usize>>) {
    let mut dist = vec![Option::None; g.len()];
    dist[start] = Some(W::zero());

    let mut heap = BinaryHeap::new();
    heap.push((Reverse(Some(W::zero())), start));

    let mut prev = vec![None; g.len()];

    while let Some((Reverse(Some(d)), cur)) = heap.pop() {
        for &(next, weight) in g[cur]
            .iter()
            .filter(|&&(x, w)| dist[x].is_none() || dist[x].unwrap() > d + w)
            .collect_vec()
        {
            dist[next] = Some(d + weight);
            heap.push((Reverse(Some(d + weight)), next));
            prev[next] = Some(cur);
        }
    }

    (dist, prev)
}
