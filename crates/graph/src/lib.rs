use itertools::{iproduct, Itertools};
use num_traits::Unsigned;
use std::{collections::VecDeque, ops::Add};

pub mod algorythm;
pub mod union_find;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UnweightedListGraph<const D: bool> {
    len: usize,
    raw_graph: Vec<Vec<usize>>,
}

pub type DULGraph = UnweightedListGraph<true>;
pub type UULGraph = UnweightedListGraph<false>;

impl<const D: bool> UnweightedListGraph<D> {
    pub fn new(len: usize) -> Self {
        Self {
            len,
            raw_graph: vec![vec![]; len],
        }
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn add_edge(&mut self, from: usize, to: usize) {
        if D {
            self.raw_graph[from].push(to);
        } else {
            self.raw_graph[from].push(to);
            self.raw_graph[to].push(from);
        }
    }

    pub fn from_edges(len: usize, edges: &[(usize, usize)]) -> Self {
        let mut graph = Self::new(len);
        for &(from, to) in edges {
            graph.add_edge(from, to);
        }
        graph
    }

    pub fn from_edges1(n: usize, edges: &[(usize, usize)]) -> Self {
        let mut graph = Self::new(n);
        for &(from, to) in edges {
            graph.add_edge(from - 1, to - 1);
        }
        graph
    }

    pub fn adjacencies(&self, from: usize) -> Vec<usize> {
        self.raw_graph[from].iter().copied().collect_vec()
    }

    pub fn distances(&self, start: usize) -> Vec<Option<usize>> {
        let mut dist = vec![None; self.len()];
        dist[start] = Some(0);
        let mut q = VecDeque::new();
        q.push_back(start);

        while let Some(cur) = q.pop_front() {
            for next in self.raw_graph[cur]
                .iter()
                .copied()
                .filter(|&x| dist[x].is_none())
                .collect_vec()
            {
                dist[next] = Some(dist[cur].unwrap() + 1);
                q.push_back(next);
            }
        }

        dist
    }
}

#[test]
fn uul_dist_test() {
    let g = UULGraph::from_edges1(5, &[(1, 2), (2, 3), (2, 4), (1, 5)]);
    let d = g.distances(0);
    dbg!(d);
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UnweightedMatrixGraph<const D: bool> {
    len: usize,
    raw_graph: Vec<Vec<bool>>,
}

pub type DUMGraph = UnweightedMatrixGraph<true>;
pub type UUMGraph = UnweightedMatrixGraph<false>;

impl<const D: bool> UnweightedMatrixGraph<D> {
    pub fn new(len: usize) -> Self {
        Self {
            len,
            raw_graph: vec![vec![false; len]; len],
        }
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn add_edge(&mut self, from: usize, to: usize) {
        if D {
            self.raw_graph[from][to] = true;
        } else {
            self.raw_graph[from][to] = true;
            self.raw_graph[to][from] = true;
        }
    }

    pub fn from_edges(len: usize, edges: &[(usize, usize)]) -> Self {
        let mut graph = Self::new(len);
        for &(from, to) in edges {
            graph.add_edge(from, to);
        }
        graph
    }

    pub fn from_edges1(len: usize, edges: &[(usize, usize)]) -> Self {
        let mut graph = Self::new(len);
        for &(from, to) in edges {
            graph.add_edge(from - 1, to - 1);
        }
        graph
    }

    pub fn adjacencies(&self, from: usize) -> Vec<usize> {
        self.raw_graph[from].iter().positions(|&x| x).collect_vec()
    }

    pub fn distances(&self, start: usize) -> Vec<Option<usize>> {
        let mut dist = vec![None; self.len()];
        dist[start] = Some(0);
        let mut q = VecDeque::new();
        q.push_back(start);

        while let Some(cur) = q.pop_front() {
            for next in self.raw_graph[cur]
                .iter()
                .positions(|&x| x)
                .filter(|&x| dist[x].is_none())
                .collect_vec()
            {
                dist[next] = dist[cur].map(|x| x + 1);
                q.push_back(next);
            }
        }

        dist
    }

    /// Floyd-Warshall algorythm
    ///
    /// Calculate distances for every pair of nodes on graph
    ///
    /// See https://en.wikipedia.org/wiki/Floyd%E2%80%93Warshall_algorithm
    pub fn floyd_warshall(&self) -> Vec<Vec<Option<usize>>> {
        let mut dist = (0..self.len())
            .map(|i| {
                (0..self.len())
                    .map(|j| {
                        if self.raw_graph[i][j] {
                            Some(1)
                        } else if i == j {
                            Some(0)
                        } else {
                            None
                        }
                    })
                    .collect_vec()
            })
            .collect_vec();

        for (k, i, j) in iproduct!(0..self.len(), 0..self.len(), 0..self.len()) {
            if let (Some(a), Some(b)) = (dist[i][k], dist[k][j]) {
                if dist[i][j].is_none() || dist[i][j].unwrap() > a + b {
                    dist[i][j] = Some(a + b);
                }
            }
        }

        dist
    }
}

#[test]
fn uum_dist_test() {
    let g = UUMGraph::from_edges1(5, &[(1, 2), (2, 3), (2, 4), (1, 5)]);
    let d = g.distances(0);
    dbg!(d);
}

#[test]
fn floyd_warshall_test() {
    let g = UUMGraph::from_edges1(5, &[(1, 2), (2, 3), (2, 4), (1, 5)]);
    let d = g.floyd_warshall();
    dbg!(d);
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WeightedListGraph<const D: bool, W> {
    len: usize,
    raw_graph: Vec<Vec<(usize, W)>>,
}

pub type DWLGraph<W> = WeightedListGraph<true, W>;
pub type UWLGraph<W> = WeightedListGraph<false, W>;

impl<const D: bool, W: Copy> WeightedListGraph<D, W> {
    pub fn new(len: usize) -> Self {
        Self {
            len,
            raw_graph: vec![vec![]; len],
        }
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn add_edge(&mut self, from: usize, to: usize, cost: W) {
        if D {
            self.raw_graph[from].push((to, cost));
        } else {
            self.raw_graph[from].push((to, cost));
            self.raw_graph[to].push((from, cost));
        }
    }

    pub fn from_edges(len: usize, edges: &[(usize, usize, W)]) -> Self {
        let mut graph = Self::new(len);
        for &(from, to, cost) in edges {
            graph.add_edge(from, to, cost);
        }
        graph
    }

    pub fn from_edges1(n: usize, edges: &[(usize, usize, W)]) -> Self {
        let mut graph = Self::new(n);
        for &(from, to, cost) in edges {
            graph.add_edge(from - 1, to - 1, cost);
        }
        graph
    }

    pub fn adjacencies(&self, from: usize) -> Vec<(usize, W)> {
        self.raw_graph[from].iter().copied().collect_vec()
    }
}

impl<const D: bool, W: Copy + Ord + Unsigned> WeightedListGraph<D, W> {
    /// Dijkstra algorythm
    ///
    /// See https://en.wikipedia.org/wiki/Dijkstra%27s_algorithm
    pub fn dijkstra(&self, start: usize) -> Vec<Option<W>> {
        crate::algorythm::dijkstra::dijkstra(&self.raw_graph, start)
    }

    /// Dijkstra algorythm with path
    ///
    /// Returns a pair of (dist, prev),
    /// to restore the shortest path from `start` to `x`,
    /// call `x = prev[x]` repeatedly
    ///
    /// See https://en.wikipedia.org/wiki/Dijkstra%27s_algorithm
    pub fn dijkstra_with_path_hints(&self, start: usize) -> (Vec<Option<W>>, Vec<Option<usize>>) {
        crate::algorythm::dijkstra::dijkstra_with_path_hint(&self.raw_graph, start)
    }
}

#[test]
fn dijkstra_test() {
    let g = UWLGraph::<u64>::from_edges1(
        6,
        &[(1, 2, 2), (2, 3, 3), (2, 4, 1), (1, 5, 100), (4, 5, 1)],
    );
    let d = g.dijkstra(0);
    dbg!(d);
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WeightedMatrixGraph<const D: bool, W> {
    len: usize,
    raw_graph: Vec<Vec<Option<W>>>,
}

pub type DWMGraph<W> = WeightedMatrixGraph<true, W>;
pub type UWMGraph<W> = WeightedMatrixGraph<false, W>;

impl<const D: bool, W: Copy> WeightedMatrixGraph<D, W> {
    pub fn new(len: usize) -> Self {
        Self {
            len,
            raw_graph: vec![vec![None; len]; len],
        }
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn add_edge(&mut self, from: usize, to: usize, cost: W) {
        if D {
            self.raw_graph[from][to] = Some(cost);
        } else {
            self.raw_graph[from][to] = Some(cost);
            self.raw_graph[to][from] = Some(cost);
        }
    }

    pub fn from_edges(len: usize, edges: &[(usize, usize, W)]) -> Self {
        let mut graph = Self::new(len);
        for &(from, to, cost) in edges {
            graph.add_edge(from, to, cost);
        }
        graph
    }

    pub fn from_edge1s(len: usize, edges: &[(usize, usize, W)]) -> Self {
        let mut graph = Self::new(len);
        for &(from, to, cost) in edges {
            graph.add_edge(from - 1, to - 1, cost);
        }
        graph
    }

    pub fn adjacencies(&self, from: usize) -> Vec<usize> {
        self.raw_graph[from]
            .iter()
            .positions(|&x| x.is_some())
            .collect_vec()
    }
}

impl<const D: bool, W: Copy + Add<Output = W> + Ord> WeightedMatrixGraph<D, W> {
    /// Floyd-Warshall algorythm
    ///
    /// Calculate distances for every pair of nodes on graph
    ///
    /// See https://en.wikipedia.org/wiki/Floyd%E2%80%93Warshall_algorithm
    pub fn floyd_warshall(&self) -> Vec<Vec<Option<W>>> {
        let mut dist = self.raw_graph.clone();

        for (k, i, j) in iproduct!(0..self.len(), 0..self.len(), 0..self.len()) {
            if let (Some(a), Some(b)) = (dist[i][k], dist[k][j]) {
                if dist[i][j].is_none() || dist[i][j].unwrap() > a + b {
                    dist[i][j] = Some(a + b);
                }
            }
        }

        dist
    }
}
