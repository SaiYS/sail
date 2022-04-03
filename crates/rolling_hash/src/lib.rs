use std::{marker::PhantomData, ops::RangeBounds};

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

pub trait Base {
    const BASE: u128;
}

#[derive(Debug, Clone)]
pub enum DefaultBase {}

impl Base for DefaultBase {
    const BASE: u128 = 20200213;
}

#[derive(Debug, Clone)]
pub struct RollingHash<B: Base = DefaultBase, M: Mod = DefaultMod> {
    phantom_base: PhantomData<fn() -> B>,
    phantom_mod: PhantomData<fn() -> M>,
    pub s: Vec<char>,
    hash: Vec<u128>,
    powers: Vec<u128>,
}

impl<B: Base, M: Mod> RollingHash<B, M> {
    pub fn new(s: Vec<char>) -> Self {
        let mut hash = vec![0u128];
        for &c in s.iter() {
            hash.push(M::rem(*hash.last().unwrap() * B::BASE + c as u128));
        }

        let mut powers = vec![1u128];
        for _ in 0..s.len() {
            powers.push(M::rem(*powers.last().unwrap() * B::BASE));
        }

        Self {
            phantom_base: PhantomData,
            phantom_mod: PhantomData,
            s,
            hash,
            powers,
        }
    }

    pub fn len(&self) -> usize {
        self.s.len()
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

pub type RollingHashDefault = RollingHash<DefaultBase, DefaultMod>;

#[test]
fn debug() {
    let s: Vec<char> = String::from("abracadabra").chars().collect();
    let h = RollingHashDefault::new(s);
    dbg!(&h);
    dbg!(h.hash(0..4));
    dbg!(h.hash(7..11));
    assert_eq!(h.hash(0..4), h.hash(7..11));
}
