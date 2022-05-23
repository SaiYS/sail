use num_traits::Zero;

#[derive(Debug, Clone)]
pub struct BellmanFordError;

type BellmanFordResult<T> = Result<T, BellmanFordError>;

pub fn bellman_ford<W: Copy + Ord + Zero>(
    n: usize,
    edges: &[(usize, usize, W)],
    start: usize,
) -> BellmanFordResult<Vec<Option<W>>> {
    let mut dist = vec![None; n];
    dist[start] = Some(W::zero());

    for _ in 1..n {
        for &(u, v, w) in edges.iter() {
            if dist[u].is_some() && dist[v].is_none() || dist[v].unwrap() > dist[u].unwrap() + w {
                dist[v] = Some(dist[u].unwrap() + w);
            }
        }
    }

    if edges.iter().all(|&(u, v, w)| match (dist[u], dist[v]) {
        (None, None) => true,
        (None, Some(_)) => true,
        (Some(_), None) => false,
        (Some(x), Some(y)) => x + w >= y,
    }) {
        Ok(dist)
    } else {
        Err(BellmanFordError)
    }
}

#[allow(clippy::type_complexity)]
pub fn bellman_ford_with_path_hint<W: Copy + Ord + Zero>(
    n: usize,
    edges: &[(usize, usize, W)],
    start: usize,
) -> BellmanFordResult<(Vec<Option<W>>, Vec<Option<usize>>)> {
    let mut dist = vec![None; n];
    dist[start] = Some(W::zero());

    let mut prev = vec![None; n];

    for _ in 1..n {
        for &(u, v, w) in edges.iter() {
            if dist[u].is_some() && dist[v].is_none() || dist[v].unwrap() > dist[u].unwrap() + w {
                dist[v] = Some(dist[u].unwrap() + w);
                prev[v] = Some(u);
            }
        }
    }

    if edges.iter().all(|&(u, v, w)| match (dist[u], dist[v]) {
        (None, None) => true,
        (None, Some(_)) => true,
        (Some(_), None) => false,
        (Some(x), Some(y)) => x + w >= y,
    }) {
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
