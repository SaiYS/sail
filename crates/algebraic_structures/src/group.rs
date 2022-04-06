use crate::monoid::MProduct;

use super::{
    monoid::{MSum, Monoid},
    semigroup::SemiGroup,
};
use num_traits::{Inv, One, Zero};
use std::ops::{Add, Mul, Neg};

pub trait Group: Monoid {
    fn inv(self) -> Self;
}

impl<T: Clone + Zero + Add<Output = T> + Neg<Output = T>> Group for MSum<T> {
    fn inv(self) -> Self {
        (-self.get()).into()
    }
}

pub type GSum<T> = MSum<T>;

impl<T: Clone + One + Mul<Output = T> + Inv<Output = T>> Group for MProduct<T> {
    fn inv(self) -> Self {
        self.get().inv().into()
    }
}

pub type GProduct<T> = MProduct<T>;
