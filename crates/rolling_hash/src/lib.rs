use std::{marker::PhantomData, ops::RangeBounds};

use rand::Rng;

pub trait Mod {
    const MOD: u128;
    fn rem(x: u128) -> u128 {
        x % Self::MOD
    }
}

#[derive(Debug, Clone)]
pub enum DefaultMod {}

impl Mod for DefaultMod {
    const MOD: u128 = (1 << 61) - 1;
    fn rem(x: u128) -> u128 {
        let res = (x >> 61) + (x & Self::MOD);
        if res < Self::MOD {
            res
        } else {
            res - Self::MOD
        }
    }
}

#[derive(Debug, Clone)]
pub struct RollingHash<M: Mod = DefaultMod> {
    phantom: PhantomData<fn() -> M>,
    pub s: Vec<char>,
    hash: Vec<u128>,
    powers: Vec<u128>,
}

impl<M: Mod> RollingHash<M> {
    pub fn new(s: Vec<char>) -> Self {
        let base: u128 = rand::thread_rng().gen_range(1, 1000000000);
        let mut hash = vec![0u128];
        for &c in s.iter() {
            hash.push(M::rem(*hash.last().unwrap() * base + c as u128));
        }

        let mut powers = vec![0u128; s.len()];
        powers[0] = 1;
        powers[1] = base;
        for i in 2..s.len() {
            powers[i] = M::rem(powers[i / 2] * powers[i % 2]);
        }

        Self {
            phantom: PhantomData,
            s,
            hash,
            powers,
        }
    }

    pub fn len(&self) -> usize {
        self.s.len()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn hash<R: RangeBounds<usize>>(&self, range: R) -> u128 {
        let (from, to) = util::expand_range_bound(&range, 0, self.len());
        let from = M::rem(self.hash[from] * self.powers[to - from]);
        let to = self.hash[to];

        M::rem(to + M::MOD - from)
    }

    pub fn is_same<R: RangeBounds<usize>>(&self, r1: R, r2: R) -> bool {
        let (from1, to1) = util::expand_range_bound(&r1, 0, self.len());
        let (from2, to2) = util::expand_range_bound(&r2, 0, self.len());

        to1 - from1 == to2 - from2 && self.hash(r1) == self.hash(r2)
    }
}

pub type RollingHashFx = RollingHash<DefaultMod>;
