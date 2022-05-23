use std::ops::{Add, Sub};

use itertools::Itertools;
use num_traits::Zero;

use crate::UWLGraph;

pub struct LowestCommonAncestor<W> {
    n: usize,
    root: usize,
    ancestor: Vec<Vec<usize>>,
    depth: Vec<usize>,
    dist: Vec<W>,
}

impl<W: Copy + Add<Output = W> + Sub<Output = W> + Zero> LowestCommonAncestor<W> {
    pub fn new(n: usize, root: usize, edges: &[(usize, usize, W)]) -> Self {
        let tree = UWLGraph::from_edges(n, edges);
        let mut parent = vec![root; n];
        let mut depth = vec![0usize; n];
        let mut dist = vec![W::zero(); n];
        let mut visited = vec![false; n];
        let mut stack = vec![(root, root)];
        while let Some((cur, prev)) = stack.pop() {
            visited[cur] = true;
            parent[cur] = prev;
            for &(next, w) in tree.raw_graph[cur].iter().filter(|&&(x, _)| !visited[x]) {
                depth[next] = depth[cur] + 1;
                dist[next] = dist[cur] + w;
                stack.push((next, cur));
            }
        }

        let mut ancestor = vec![parent];
        for k in 1.. {
            if ancestor.last().unwrap().iter().all(|&x| x == root) {
                break;
            }

            let next = (0..n)
                .map(|x| ancestor[k - 1][ancestor[k - 1][x]])
                .collect_vec();
            ancestor.push(next);
        }

        Self {
            n,
            root,
            ancestor,
            depth,
            dist,
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

    pub fn lca(&self, mut a: usize, mut b: usize) -> usize {
        let mut da = self.depth[a];
        let mut db = self.depth[b];
        if da > db {
            std::mem::swap(&mut a, &mut b);
            std::mem::swap(&mut da, &mut db);
        }

        let mut d = db - da;
        let mut i = 0;
        while d != 0 {
            if d & 1 != 0 {
                b = self.ancestor[i][b];
            }
            d >>= 1;
            i += 1;
        }

        if a == b {
            a
        } else {
            for k in (0..self.ancestor.len()).rev() {
                if self.ancestor[k][a] != self.ancestor[k][b] {
                    a = self.ancestor[k][a];
                    b = self.ancestor[k][b];
                }
            }
            debug_assert_eq!(self.ancestor[0][a], self.ancestor[0][b]);
            self.ancestor[0][a]
        }
    }

    pub fn distance(&self, a: usize, b: usize) -> W {
        self.dist[a] + self.dist[b] - self.dist[self.lca(a, b)] - self.dist[self.lca(a, b)]
    }
}

#[test]
fn debug() {
    let edges = [(0, 1, 1), (0, 2, 1), (1, 3, 1), (1, 4, 1), (3, 5, 1)];
    let lca = LowestCommonAncestor::new(6, 0, &edges);

    assert_eq!(lca.lca(2, 3), 0);
    assert_eq!(lca.lca(3, 4), 1);
    assert_eq!(lca.lca(3, 5), 3);
    assert_eq!(lca.lca(5, 4), 1);

    assert_eq!(lca.distance(0, 2), 1);
    assert_eq!(lca.distance(1, 2), 2);
    assert_eq!(lca.distance(0, 3), 2);
    assert_eq!(lca.distance(4, 5), 3);
}
