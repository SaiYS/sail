use itertools::Itertools;

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

    pub fn from_edge1s(len: usize, edges: &[(usize, usize)]) -> Self {
        let mut graph = Self::new(len);
        for &(from, to) in edges {
            graph.add_edge(from - 1, to - 1);
        }
        graph
    }

    pub fn adjacencies(&self, from: usize) -> Vec<usize> {
        self.buffer[from].iter().positions(|&x| x).collect_vec()
    }
}
