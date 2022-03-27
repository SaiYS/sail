use super::{Directed, DirectionType, Undirected};
use std::marker::PhantomData;

#[derive(Debug, Clone)]
pub struct ListGraph<D> {
    pub(crate) phantom: PhantomData<D>,
    pub(crate) len: usize,
    pub(crate) data: Vec<Vec<usize>>,
}

pub type DirectedListGraph = ListGraph<Directed>;

pub type UndirectedListGraph = ListGraph<Undirected>;

impl<D: DirectionType> ListGraph<D> {
    pub fn len(&self) -> usize {
        self.len
    }

    pub(crate) fn directed(&self) -> bool {
        D::DIRECTED
    }

    pub fn new(n: usize) -> Self {
        let data = vec![vec![]; n];

        Self {
            phantom: PhantomData,
            len: n,
            data,
        }
    }

    pub fn add_edge(&mut self, from: usize, to: usize) {
        if self.directed() {
            self.data[from].push(to);
        } else {
            self.data[from].push(to);
            self.data[to].push(from);
        }
    }

    pub fn from_edges(n: usize, edges: &[(usize, usize)]) -> Self {
        let mut graph = Self::new(n);
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

    pub fn adjacencies(&self, node: usize) -> std::slice::Iter<usize> {
        self.data[node].iter()
    }
}
