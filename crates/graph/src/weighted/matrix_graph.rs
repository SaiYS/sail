use std::ops::Add;

use itertools::{iproduct, Itertools};

use crate::Distance::{self, Reachable, Unreachable};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MatrixGraph<const D: bool, W> {
    len: usize,
    buffer: Vec<Vec<Distance<W>>>,
}

pub type DWMGraph = MatrixGraph<true, u64>;
pub type UWMGraph = MatrixGraph<false, u64>;

impl<const D: bool, W: Copy> MatrixGraph<D, W> {
    pub fn new(len: usize) -> Self {
        Self {
            len,
            buffer: vec![vec![Unreachable; len]; len],
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
            self.buffer[from][to] = Reachable(cost);
        } else {
            self.buffer[from][to] = Reachable(cost);
            self.buffer[to][from] = Reachable(cost);
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
        self.buffer[from]
            .iter()
            .positions(|&x| x.is_reachable())
            .collect_vec()
    }
}

impl<const D: bool, W: Copy + Add<Output = W> + Ord> MatrixGraph<D, W> {
    /// Floyd-Warshall algorythm
    ///
    /// Calculate distances for every pair of nodes on graph
    ///
    /// See https://en.wikipedia.org/wiki/Floyd%E2%80%93Warshall_algorithm
    pub fn floyd_warshall(&self) -> Vec<Vec<Distance<W>>> {
        let mut dist = self.buffer.clone();

        for (k, i, j) in iproduct!(0..self.len(), 0..self.len(), 0..self.len()) {
            if dist[i][j] > dist[i][k] + dist[k][j] {
                dist[i][j] = dist[i][k] + dist[k][j];
            }
        }

        dist
    }
}
