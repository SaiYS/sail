use super::Generable;
use itertools::Itertools;
use rand::seq::SliceRandom;

#[derive(Clone, Copy)]
pub struct Permutation(pub usize);

impl Generable for Permutation {
    type Output = Vec<usize>;

    fn gen<R: rand::Rng>(self, rng: &mut R) -> Vec<usize> {
        let mut res = (1..=self.0).collect_vec();
        res.shuffle(rng);
        res
    }
}

#[derive(Clone, Copy)]
pub struct Permutation1(pub usize);

impl Generable for Permutation1 {
    type Output = Vec<usize>;

    fn gen<R: rand::Rng>(self, rng: &mut R) -> Vec<usize> {
        let mut res = (0..self.0).collect_vec();
        res.shuffle(rng);
        res
    }
}

#[derive(Clone, Copy)]
pub struct StarGraphEdges(pub usize);

impl Generable for StarGraphEdges {
    type Output = Vec<(usize, usize)>;

    fn gen<R: rand::Rng>(self, rng: &mut R) -> Self::Output {
        let root = rng.gen_range(1, self.0 + 1);
        (1..=self.0)
            .filter(|&x| x != root)
            .map(|x| (root, x))
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        gen,
        marker::{Permutation, StarGraphEdges},
        Generable,
    };

    #[test]
    fn permutation() {
        gen! {
            n = 10usize,
            p = Permutation(n)
        }

        dbg!(p);
    }

    #[test]
    fn stargraph() {
        gen! {
            n = 5usize,
            e = StarGraphEdges(n)
        }

        dbg!(e);
    }
}
