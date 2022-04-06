use std::ops::{Add, Mul, Neg};

use num_traits::{Inv, One, Zero};

use super::group::{GProduct, GSum, Group};

pub trait AbelianGroup: Group {}

impl<T: Clone + Zero + Add<Output = T> + Neg<Output = T>> AbelianGroup for ASum<T> {}

pub type ASum<T> = GSum<T>;

impl<T: Clone + One + Mul<Output = T> + Inv<Output = T>> AbelianGroup for AProduct<T> {}

pub type AProduct<T> = GProduct<T>;
