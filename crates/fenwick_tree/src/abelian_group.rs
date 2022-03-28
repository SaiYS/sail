use std::ops::{Add, Sub};

use num_traits::Zero;

pub trait AbelianGroup: Clone {
    type T;
    fn identity() -> Self;
    fn get(self) -> Self::T;
    fn set(value: Self::T) -> Self;
    fn add(x: Self, y: Self) -> Self;
    fn inv(self) -> Self;
    // associativity: x + (y + z) == (x + y) + z
    // commutativity: x + y == y + x
}

impl<T: Add<Output = T>> Add<Self> for Addictive<T> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0)
    }
}

#[derive(Debug, Clone)]
pub struct Addictive<T>(T);

impl<T> AbelianGroup for Addictive<T>
where
    T: Add<Output = T> + Sub<Output = T> + Zero + Clone,
{
    type T = T;

    fn identity() -> Self {
        Addictive(T::zero())
    }

    fn get(self) -> Self::T {
        self.0
    }

    fn set(value: T) -> Self {
        Self(value)
    }

    fn add(x: Self, y: Self) -> Self {
        x + y
    }

    fn inv(self) -> Self {
        Self(T::zero() - self.get())
    }
}
