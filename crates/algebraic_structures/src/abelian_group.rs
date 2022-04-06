use super::group::{GProduct, GSum, Group};
use num_traits::{Inv, One, Zero};
use std::ops::{Add, Mul, Neg};

pub trait AbelianGroup: Group {}

impl<T: Clone + Zero + Add<Output = T> + Neg<Output = T>> AbelianGroup for GSum<T> {}

pub type ASum<T> = GSum<T>;

impl<T: Clone + One + Mul<Output = T> + Inv<Output = T>> AbelianGroup for GProduct<T> {}

pub type AProduct<T> = GProduct<T>;
