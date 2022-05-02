use itertools::Itertools;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ListGraph<const D: bool> {
    len: usize,
    buffer: Vec<Vec<usize>>,
}

pub type DULGraph = ListGraph<true>;

pub type UULGraph = ListGraph<false>;

impl<const D: bool> ListGraph<D> {
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

    pub fn add_edge(&mut self, from: usize, to: usize) {
        if D {
            self.buffer[from].push(to);
        } else {
            self.buffer[from].push(to);
            self.buffer[to].push(from);
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
        self.buffer[from].iter().copied().collect_vec()
    }
}
