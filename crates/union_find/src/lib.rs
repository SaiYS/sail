#[derive(Debug, Clone)]
pub struct UnionFind {
    root: Vec<Option<usize>>,
    size: Vec<usize>,
}

impl UnionFind {
    pub fn new(n: usize) -> Self {
        Self {
            root: vec![None; n],
            size: vec![1; n],
        }
    }

    pub fn tree_size(&mut self, i: usize) -> usize {
        let root = self.root(i);
        self.size[root]
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
    }

    pub fn is_joint(&mut self, a: usize, b: usize) -> bool {
        self.root(a) == self.root(b)
    }
}
