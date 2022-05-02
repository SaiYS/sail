use crate::Distance::{self, Reachable, Unreachable};
use num_traits::Num;

#[derive(Debug, Clone)]
pub struct BellmanFordError;

type BellmanFordResult<T> = Result<T, BellmanFordError>;

pub fn bellman_ford<W: Copy + Ord + Num>(
    n: usize,
    edges: &[(usize, usize, W)],
    start: usize,
) -> BellmanFordResult<Vec<Distance<W>>> {
    let mut dist = vec![Unreachable; n];
    dist[start] = Reachable(W::zero());

    for _ in 1..n {
        for &(u, v, w) in edges.iter() {
            if dist[v] > dist[u] + w {
                dist[v] = dist[u] + w;
            }
        }
    }

    if edges.iter().all(|&(u, v, w)| dist[u] + w >= dist[v]) {
        Ok(dist)
    } else {
        Err(BellmanFordError)
    }
}

#[allow(clippy::type_complexity)]
pub fn bellman_ford_with_path_hint<W: Copy + Ord + Num>(
    n: usize,
    edges: &[(usize, usize, W)],
    start: usize,
) -> BellmanFordResult<(Vec<Distance<W>>, Vec<Option<usize>>)> {
    let mut dist = vec![Distance::Unreachable; n];
    dist[start] = Reachable(W::zero());

    let mut prev = vec![None; n];

    for _ in 1..n {
        for &(u, v, w) in edges.iter() {
            if dist[v] > dist[u] + w {
                dist[v] = dist[u] + w;
                prev[v] = Some(u);
            }
        }
    }

    if edges.iter().all(|&(u, v, w)| dist[u] + w >= dist[v]) {
        Ok((dist, prev))
    } else {
        Err(BellmanFordError)
    }
}

#[test]
fn bellman_ford_test() {
    let negetive_cycle: [(usize, usize, i32); 3] = [(0, 1, 1), (1, 2, 1), (2, 0, -100)];
    assert!(bellman_ford(3, &negetive_cycle, 0).is_err());

    let edges: [(usize, usize, i32); 5] = [(0, 1, 1), (0, 2, 3), (1, 2, -1), (1, 3, 3), (2, 3, 1)];
    let dist = bellman_ford(4, &edges, 0).unwrap();
    dbg!(dist);
}
