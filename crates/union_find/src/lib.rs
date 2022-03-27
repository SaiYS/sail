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

    fn find_root(&mut self, id: usize) -> usize {
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

#[test]
fn feature() {
    let n = 5;
    let mut uf = UnionFind::new(n);
    dbg!(&uf);

    uf.unite(0, 1);
    assert!(uf.is_joint(0, 1));
    assert!(!uf.is_joint(1, 2));
    assert!(!uf.is_joint(2, 0));
    dbg!(&uf);

    uf.unite(1, 2);
    assert!(uf.is_joint(0, 1));
    assert!(uf.is_joint(1, 2));
    assert!(uf.is_joint(2, 0));
    assert!(!uf.is_joint(0, 3));
    assert!(!uf.is_joint(1, 3));
    assert!(!uf.is_joint(2, 3));
    dbg!(&uf);

    uf.unite(3, 4);
    dbg!(&uf);

    uf.unite(1, 4);
    dbg!(&uf);
    uf.unite(2, 2);
    dbg!(&uf);
}
