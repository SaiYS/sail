use crate::Distance::{self, Reachable, Unreachable};
use itertools::{iproduct, Itertools};
use std::collections::VecDeque;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MatrixGraph<const D: bool> {
    len: usize,
    buffer: Vec<Vec<bool>>,
}

pub type DUMGraph = MatrixGraph<true>;
pub type UUMGraph = MatrixGraph<false>;

impl<const D: bool> MatrixGraph<D> {
    pub fn new(len: usize) -> Self {
        Self {
            len,
            buffer: vec![vec![false; len]; len],
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
            self.buffer[from][to] = true;
        } else {
            self.buffer[from][to] = true;
            self.buffer[to][from] = true;
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
        self.buffer[from].iter().positions(|&x| x).collect_vec()
    }

    pub fn distances(&self, start: usize) -> Vec<Distance<usize>> {
        let mut dist = vec![Unreachable; self.len()];
        dist[start] = Reachable(0);
        let mut q = VecDeque::new();
        q.push_back(start);

        while let Some(cur) = q.pop_front() {
            for next in self.buffer[cur]
                .iter()
                .positions(|&x| x)
                .filter(|&x| dist[x].is_unreachable())
                .collect_vec()
            {
                dist[next] = dist[cur] + 1;
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
    pub fn floyd_warshall(&self) -> Vec<Vec<Distance<usize>>> {
        let mut dist = (0..self.len())
            .map(|i| {
                (0..self.len())
                    .map(|j| {
                        if self.buffer[i][j] {
                            Reachable(1)
                        } else if i == j {
                            Reachable(0)
                        } else {
                            Unreachable
                        }
                    })
                    .collect_vec()
            })
            .collect_vec();

        for (k, i, j) in iproduct!(0..self.len(), 0..self.len(), 0..self.len()) {
            if dist[i][j] > dist[i][k] + dist[k][j] {
                dist[i][j] = dist[i][k] + dist[k][j];
            }
        }

        dist
    }
}

#[test]
fn dist_test() {
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
