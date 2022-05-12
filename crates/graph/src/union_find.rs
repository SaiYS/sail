use std::collections::HashMap;

use itertools::Itertools;

/// UnionFind (Disjoint Set Union)
#[derive(Debug, Clone)]
pub struct UnionFind {
    len: usize,
    count: usize,
    root: Vec<Option<usize>>,
    size: Vec<usize>,
}

impl UnionFind {
    pub fn new(n: usize) -> Self {
        Self {
            len: n,
            count: n,
            root: vec![None; n],
            size: vec![1; n],
        }
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn root(&mut self, i: usize) -> usize {
        if let Some(parent) = self.root[i] {
            let res = self.root(parent);
            self.root[i] = Some(res);
            res
        } else {
            i
        }
    }

    /// Merges trees containing `a` or `b`
    ///
    /// if `a` and `b` are already connected, do nothing
    pub fn unite(&mut self, a: usize, b: usize) {
        let mut a = self.root(a);
        let mut b = self.root(b);

        if a == b {
            return;
        }

        let size_a = self.size[a];
        let size_b = self.size[b];

        if size_a < size_b {
            std::mem::swap(&mut a, &mut b);
        }

        self.root[b] = Some(a);
        self.size[a] += self.size[b];
        self.count -= 1;
    }

    /// Returns if `a` and `b` is contained in the same connected tree
    pub fn is_joint(&mut self, a: usize, b: usize) -> bool {
        self.root(a) == self.root(b)
    }

    pub fn tree_size(&mut self, i: usize) -> usize {
        let root = self.root(i);
        self.size[root]
    }

    pub fn get_tree(&mut self, i: usize) -> Vec<usize> {
        (0..self.len())
            .filter(|&x| self.root(x) == self.root(i))
            .collect_vec()
    }

    /// Returns the number of connected trees
    pub fn trees_count(&mut self) -> usize {
        self.count
    }

    /// Returns map of connected trees
    pub fn trees(&mut self) -> HashMap<usize, Vec<usize>> {
        (0..self.len()).map(|x| (self.root(x), x)).into_group_map()
    }
}
