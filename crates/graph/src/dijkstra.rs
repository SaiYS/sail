use crate::Distance::{self, Reachable};
use itertools::Itertools;
use num_traits::Unsigned;
use std::{cmp::Reverse, collections::BinaryHeap};

pub fn dijkstra<W: Copy + Ord + Unsigned>(g: &[Vec<(usize, W)>], start: usize) -> Vec<Distance<W>> {
    let mut dist = vec![Distance::Unreachable; g.len()];
    dist[start] = Reachable(W::zero());

    let mut heap = BinaryHeap::new();
    heap.push((Reverse(Reachable(W::zero())), start));

    while let Some((Reverse(d), cur)) = heap.pop() {
        for &(next, weight) in g[cur]
            .iter()
            .filter(|&&(x, w)| d + w < dist[x])
            .collect_vec()
        {
            dist[next] = d + weight;
            heap.push((Reverse(d + weight), next));
        }
    }

    dist
}

pub fn dijkstra_with_path_hint<W: Copy + Ord + Unsigned>(
    g: &[Vec<(usize, W)>],
    start: usize,
) -> (Vec<Distance<W>>, Vec<Option<usize>>) {
    let mut dist = vec![Distance::Unreachable; g.len()];
    dist[start] = Reachable(W::zero());

    let mut heap = BinaryHeap::new();
    heap.push((Reverse(Reachable(W::zero())), start));

    let mut prev = vec![None; g.len()];

    while let Some((Reverse(d), cur)) = heap.pop() {
        for &(next, weight) in g[cur]
            .iter()
            .filter(|&&(x, w)| d + w < dist[x])
            .collect_vec()
        {
            dist[next] = d + weight;
            heap.push((Reverse(d + weight), next));
            prev[next] = Some(cur);
        }
    }

    (dist, prev)
}
