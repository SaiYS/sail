use crate::Distance;
use itertools::Itertools;
use num_traits::Unsigned;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ListGraph<const D: bool, W> {
    len: usize,
    buffer: Vec<Vec<(usize, W)>>,
}

pub type DWLGraph = ListGraph<true, u64>;
pub type UWLGraph = ListGraph<false, u64>;

impl<const D: bool, W: Copy> ListGraph<D, W> {
    pub fn new(len: usize) -> Self {
        Self {
            len,
            buffer: vec![vec![]; len],
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
            self.buffer[from].push((to, cost));
        } else {
            self.buffer[from].push((to, cost));
            self.buffer[to].push((from, cost));
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
        self.buffer[from].iter().copied().collect_vec()
    }
}

impl<const D: bool, W: Copy + Ord + Unsigned> ListGraph<D, W> {
    /// Dijkstra algorythm
    ///
    /// See https://en.wikipedia.org/wiki/Dijkstra%27s_algorithm
    pub fn dijkstra(&self, start: usize) -> Vec<Distance<W>> {
        crate::dijkstra::dijkstra(&self.buffer, start)
    }

    /// Dijkstra algorythm with path
    ///
    /// Returns a pair of (dist, prev),
    /// to restore the shortest path from `start` to `x`,
    /// call `x = prev[x]` repeatedly
    ///
    /// See https://en.wikipedia.org/wiki/Dijkstra%27s_algorithm
    pub fn dijkstra_with_path_hints(&self, start: usize) -> (Vec<Distance<W>>, Vec<Option<usize>>) {
        crate::dijkstra::dijkstra_with_path_hint(&self.buffer, start)
    }
}

#[test]
fn dijkstra_test() {
    let g = UWLGraph::from_edges1(
        6,
        &[(1, 2, 2), (2, 3, 3), (2, 4, 1), (1, 5, 100), (4, 5, 1)],
    );
    let d = g.dijkstra(0);
    dbg!(d);
}
