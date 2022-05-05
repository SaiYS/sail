use crate::union_find::UnionFind;
use itertools::Itertools;

/// Kruskal's algorythm
///
/// Returns the edges of minimum spanning tree of a given weighted graph
///
/// See https://en.wikipedia.org/wiki/Kruskal%27s_algorithm
pub fn kuruskal(n: usize, edges: &[(usize, usize, usize)]) -> Vec<(usize, usize, usize)> {
    let mut uf = UnionFind::new(n);
    let mut mst_edges = Vec::new();

    for &(a, b, c) in edges.iter().sorted_by_key(|x| x.2) {
        if !uf.is_joint(a, b) {
            uf.unite(a, b);
            mst_edges.push((a, b, c));
        }

        if mst_edges.len() == n - 1 {
            break;
        }
    }

    mst_edges
}

#[test]
fn kruskal_test() {
    let edges = vec![
        (0, 1, 2),
        (1, 2, 3),
        (0, 2, 6),
        (1, 3, 5),
        (3, 4, 9),
        (2, 4, 8),
    ];

    let mst = kuruskal(5, &edges);
    dbg!(mst);
}
