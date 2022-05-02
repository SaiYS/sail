use itertools::Itertools;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MatrixGraph<const D: bool, W> {
    len: usize,
    buffer: Vec<Vec<Option<W>>>,
}

pub type DWMGraph = MatrixGraph<true, i64>;
pub type UWMGraph = MatrixGraph<false, i64>;

impl<const D: bool, W: Copy> MatrixGraph<D, W> {
    pub fn new(len: usize) -> Self {
        Self {
            len,
            buffer: vec![vec![None; len]; len],
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
            self.buffer[from][to] = Some(cost);
        } else {
            self.buffer[from][to] = Some(cost);
            self.buffer[to][from] = Some(cost);
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
            .positions(|&x| x.is_some())
            .collect_vec()
    }
}
