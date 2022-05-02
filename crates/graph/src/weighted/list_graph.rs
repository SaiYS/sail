use itertools::Itertools;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ListGraph<const D: bool, W> {
    len: usize,
    buffer: Vec<Vec<(usize, W)>>,
}

pub type DWLGraph = ListGraph<true, i64>;

pub type UWLGraph = ListGraph<false, i64>;

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
