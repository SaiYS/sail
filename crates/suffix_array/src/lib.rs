use std::{fmt::Debug, marker::PhantomData, ops::Index};

use induced_sort::InducedSort;
use itertools::Itertools;

pub trait SuffixSort {
    fn sort(s: &[u8]) -> Vec<usize>;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum LS {
    L,
    S,
    LMS,
}

impl LS {
    fn is_s(self) -> bool {
        match self {
            LS::L => false,
            LS::S => true,
            LS::LMS => true,
        }
    }
    fn is_l(self) -> bool {
        match self {
            LS::L => true,
            LS::S => false,
            LS::LMS => false,
        }
    }

    fn is_lms(self) -> bool {
        match self {
            LS::L => false,
            LS::S => false,
            LS::LMS => true,
        }
    }
}

#[derive(Debug, Clone)]
pub enum DefaultSort {}

impl SuffixSort for DefaultSort {
    fn sort(s: &[u8]) -> Vec<usize> {
        (0..s.len()).sorted_by_key(|&from| &s[from..]).collect_vec()
    }
}

pub mod induced_sort;

#[derive(Debug, Clone)]
pub struct SuffixArray<S> {
    phantom: PhantomData<fn() -> S>,
    buffer: Vec<usize>,
    pub s: Vec<char>,
}

impl<S: SuffixSort> SuffixArray<S> {
    pub fn new(s: Vec<char>) -> Self {
        Self {
            phantom: PhantomData,
            buffer: S::sort(&s.iter().map(|&x| x as u8).collect_vec()),
            s,
        }
    }
}

impl<S> SuffixArray<S> {
    pub fn len(&self) -> usize {
        self.s.len()
    }

    pub fn find<T: AsRef<str>>(&self, pat: T) -> Option<usize> {
        let pat = pat.as_ref().chars().collect_vec();

        let p = if &self[0] > &pat {
            0
        } else {
            let mut l = 0;
            let mut r = self.len();
            loop {
                let m = (l + r) / 2;
                if &self[m] < &pat {
                    l = m;
                } else {
                    r = m;
                }
                if r == l + 1 {
                    break;
                }
            }
            r
        };

        if self[p].starts_with(&pat) {
            Some(self.buffer[p])
        } else {
            None
        }
    }
}

impl<S> Index<usize> for SuffixArray<S> {
    type Output = [char];

    fn index(&self, index: usize) -> &Self::Output {
        &self.s[self.buffer[index]..]
    }
}

pub type SaIs = SuffixArray<InducedSort>;
