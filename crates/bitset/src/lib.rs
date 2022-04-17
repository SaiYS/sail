const ZEROS: u8 = 0u8;
const ONES: u8 = std::u8::MAX;

use std::{
    fmt::Debug,
    mem::swap,
    ops::{BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Not},
};

#[derive(Default, Clone)]
pub struct Bitset {
    len: usize,
    buffer: Box<[u8]>,
}

impl From<Vec<bool>> for Bitset {
    fn from(v: Vec<bool>) -> Self {
        let mut res = Self::new(v.len());
        for i in 0..v.len() {
            if v[i] {
                res.set(i);
            }
        }
        res
    }
}

impl ToString for Bitset {
    fn to_string(&self) -> String {
        (0..self.len())
            .map(|x| {
                if {
                    let (w, b) = address(x);
                    self.buffer[w] & (1 << b) != 0
                } {
                    '1'
                } else {
                    '0'
                }
            })
            .collect::<String>()
    }
}

impl Debug for Bitset {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{}]", self.to_string())
    }
}

impl BitAnd<Self> for Bitset {
    type Output = Self;

    fn bitand(mut self, mut rhs: Self) -> Self::Output {
        if self.len() < rhs.len() {
            swap(&mut self, &mut rhs);
        }
        for w in 0..self.words() {
            self.buffer[w] &= rhs.buffer.get(w).unwrap_or(&ZEROS);
        }
        self
    }
}

impl BitAndAssign for Bitset {
    fn bitand_assign(&mut self, rhs: Self) {
        *self = self.clone() & rhs
    }
}

impl BitXor<Self> for Bitset {
    type Output = Self;

    fn bitxor(mut self, mut rhs: Self) -> Self::Output {
        if self.len() < rhs.len() {
            swap(&mut self, &mut rhs);
        }
        for i in 0..self.len() {
            if rhs.get(i).unwrap_or(false) {
                self.flip(i);
            }
        }
        self
    }
}

impl BitXorAssign for Bitset {
    fn bitxor_assign(&mut self, rhs: Self) {
        *self = self.clone() ^ rhs
    }
}

impl BitOr<Self> for Bitset {
    type Output = Self;

    fn bitor(mut self, mut rhs: Self) -> Self::Output {
        if self.len() < rhs.len() {
            swap(&mut self, &mut rhs);
        }
        for i in 0..self.len() {
            if rhs.get(i).unwrap_or(false) {
                self.set(i);
            }
        }
        self
    }
}

impl BitOrAssign for Bitset {
    fn bitor_assign(&mut self, rhs: Self) {
        *self = self.clone() | rhs
    }
}

impl Not for Bitset {
    type Output = Self;

    fn not(mut self) -> Self::Output {
        (0..self.len()).for_each(|x| {
            self.flip(x);
        });
        self
    }
}

impl Bitset {
    pub fn new(n: usize) -> Self {
        debug_assert!(n != 0);
        Self {
            len: n,
            buffer: vec![ZEROS; (n - 1 >> 3) + 1].into_boxed_slice(),
        }
    }

    pub fn len(&self) -> usize {
        self.len
    }

    fn words(&self) -> usize {
        self.buffer.len()
    }

    pub fn get(&self, i: usize) -> Option<bool> {
        if i < self.len() {
            let (w, b) = address(i);
            Some(self.buffer[w] & (1 << b) != 0)
        } else {
            None
        }
    }

    pub fn set(&mut self, i: usize) {
        debug_assert!(i < self.len());
        let (w, b) = address(i);
        self.buffer[w] |= 1 << b;
    }

    pub fn remove(&mut self, i: usize) {
        debug_assert!(i < self.len());
        let (w, b) = address(i);
        self.buffer[w] &= ONES ^ (1 << b);
    }

    pub fn flip(&mut self, i: usize) {
        debug_assert!(i < self.len());
        let (w, b) = address(i);
        self.buffer[w] ^= 1 << b;
    }

    pub fn count_zeros(&self) -> usize {
        self.len() - self.count_ones()
    }

    pub fn count_ones(&self) -> usize {
        self.buffer
            .iter()
            .map(|w| w.count_ones() as usize)
            .sum::<usize>()
            - (self.len()..self.words() * 8)
                .filter(|&x| {
                    let (w, b) = address(x);
                    self.buffer[w] & (1 << b) != 0
                })
                .count()
    }
}

// return (w, b), kth bit appears at bth bit of wth word
fn address(i: usize) -> (usize, usize) {
    (i >> 3, i & 7)
}

#[test]
fn feature() {
    let mut a = Bitset::new(16);
    for i in 0..10 {
        a.set(i);
    }
    dbg!(&a);
    dbg!(a.count_zeros());
}
