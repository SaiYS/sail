use crate::UWLGraph;
use itertools::{izip, Itertools};
use num_traits::Zero;
use segment_tree::{segment_tree::SegmentTree, Min};
use std::ops::{Add, Sub};

#[derive(Clone)]
pub struct LowestCommonAncestor<W> {
    n: usize,
    root: usize,
    dist: Vec<W>,
    first_appear: Vec<usize>,
    rmq: SegmentTree<Min<(usize, usize)>>,
}

impl<W: Copy + Add<Output = W> + Sub<Output = W> + Zero> LowestCommonAncestor<W> {
    pub fn new(n: usize, root: usize, edges: &[(usize, usize, W)]) -> Self {
        let mut tree = UWLGraph::<W>::new(n);
        for &(u, v, c) in edges {
            tree.add_edge(u, v, c);
        }
        let mut dist = vec![W::zero(); n];

        let mut depth = vec![];
        let mut eular_tour = vec![];
        let mut first_appear = vec![None; n];

        dfs(
            &tree,
            DfsRecord {
                eular_tour: &mut eular_tour,
                depth: &mut depth,
                first_appear: &mut first_appear,
                dist: &mut dist,
            },
            root,
            None,
            0,
            Zero::zero(),
        );

        Self {
            n,
            root,
            dist,
            first_appear: first_appear.into_iter().flatten().collect_vec(),
            rmq: SegmentTree::from(izip!(depth, eular_tour).collect_vec()),
        }
    }

    pub fn len(&self) -> usize {
        self.n
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

    pub fn distance(&self, a: usize, b: usize) -> W {
        self.dist[a] + self.dist[b] - self.dist[self.lca(a, b)] - self.dist[self.lca(a, b)]
    }
}

struct DfsRecord<'a, W> {
    eular_tour: &'a mut Vec<usize>,
    depth: &'a mut Vec<usize>,
    first_appear: &'a mut [Option<usize>],
    dist: &'a mut [W],
}

fn dfs<W: Copy + Add<Output = W> + Zero>(
    graph: &UWLGraph<W>,
    DfsRecord {
        eular_tour,
        depth,
        first_appear,
        dist,
    }: DfsRecord<W>,
    cur: usize,
    prev: Option<usize>,
    d: usize,
    w: W,
) {
    first_appear[cur] = Some(eular_tour.len());
    eular_tour.push(cur);
    depth.push(d);
    if let Some(p) = prev {
        dist[cur] = dist[p] + w;
    }
    for &(next, next_w) in graph.list[cur].iter().filter(|&&(x, _)| Some(x) != prev) {
        dfs(
            graph,
            DfsRecord {
                eular_tour,
                depth,
                first_appear,
                dist,
            },
            next,
            Some(cur),
            d + 1,
            next_w,
        );
        eular_tour.push(cur);
        depth.push(d);
    }
}

#[test]
fn lca_test() {
    let edges = [(0, 1, 1), (0, 2, 1), (1, 3, 1), (1, 4, 1), (3, 5, 1)];
    let lca = LowestCommonAncestor::new(6, 0, &edges);

    assert_eq!(lca.lca(2, 3), 0);
    assert_eq!(lca.lca(5, 4), 1);
    assert_eq!(lca.lca(3, 5), 3);
    assert_eq!(lca.lca(3, 4), 1);

    assert_eq!(lca.distance(0, 2), 1);
    assert_eq!(lca.distance(1, 2), 2);
    assert_eq!(lca.distance(0, 3), 2);
    assert_eq!(lca.distance(4, 5), 3);
}
