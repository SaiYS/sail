use itertools::{izip, Itertools};
use segment_tree::{segment_tree::SegmentTree, Min};

#[derive(Clone)]
pub struct LowestCommonAncestor {
    len: usize,
    root: usize,
    first_appear: Vec<usize>,
    rmq: SegmentTree<Min<(usize, usize)>>,
}

impl LowestCommonAncestor {
    pub fn new(len: usize, root: usize, edges: &[(usize, usize)]) -> Self {
        let mut g = vec![Vec::new(); len];
        for &(u, v) in edges {
            g[u].push(v);
            g[v].push(u);
        }

        let mut depth = vec![];
        let mut eular_tour = vec![];
        let mut first_appear = vec![None; len];

        fn dfs(
            g: &[Vec<usize>],
            eular_tour: &mut Vec<usize>,
            depth: &mut Vec<usize>,
            first_appear: &mut [Option<usize>],
            cur: usize,
            prev: Option<usize>,
            d: usize,
        ) {
            first_appear[cur] = Some(eular_tour.len());
            eular_tour.push(cur);
            depth.push(d);
            for &next in g[cur].iter().filter(|&&x| Some(x) != prev) {
                dfs(g, eular_tour, depth, first_appear, next, Some(cur), d + 1);
                eular_tour.push(cur);
                depth.push(d);
            }
        }

        dfs(
            &g,
            &mut eular_tour,
            &mut depth,
            &mut first_appear,
            root,
            None,
            0,
        );

        Self {
            len,
            root,
            first_appear: first_appear.into_iter().flatten().collect_vec(),
            rmq: SegmentTree::from(izip!(depth, eular_tour).collect_vec()),
        }
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn root(&self) -> usize {
        self.root
    }

    pub fn lca(&self, a: usize, b: usize) -> usize {
        let mut a = self.first_appear[a];
        let mut b = self.first_appear[b];
        if b < a {
            std::mem::swap(&mut a, &mut b);
        }
        self.rmq.range(a..=b).1
    }
}

#[test]
fn lca_test() {
    let edges = [(0, 1), (0, 2), (1, 3), (1, 4), (3, 5)];
    let lca = LowestCommonAncestor::new(6, 0, &edges);

    assert_eq!(lca.lca(2, 3), 0);
    assert_eq!(lca.lca(5, 4), 1);
    assert_eq!(lca.lca(3, 5), 3);
    assert_eq!(lca.lca(3, 4), 1);
}
