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

    pub fn tree_size(&mut self, id: usize) -> usize {
        let root = self.find_root(id);
        self.size[root]
    }

    pub fn find_root(&mut self, id: usize) -> usize {
        if let Some(parent) = self.root[id] {
            let res = self.find_root(parent);
            self.root[id] = Some(res);
            res
        } else {
            id
        }
    }

    pub fn unite(&mut self, a: usize, b: usize) {
        let mut a = self.find_root(a);
        let mut b = self.find_root(b);

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
        self.find_root(a) == self.find_root(b)
    }
}

