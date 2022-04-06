use num_integer::Integer;
use num_traits::{Bounded, One, Zero};
use std::ops::{Add, BitAnd, BitOr, BitXor, Mul, Not};

pub trait SemiGroup: Clone + From<Self::T> {
    type T: Clone;
    fn binary_operation(x: Self, y: Self) -> Self;
    fn get(self) -> Self::T;
}

#[derive(Debug, Clone, Copy)]
pub struct SMax<T: Ord + Bounded>(T);

impl<T: Clone + Ord + Bounded> SemiGroup for SMax<T> {
    type T = T;

    fn binary_operation(x: Self, y: Self) -> Self {
        x.get().max(y.get()).into()
    }

    fn get(self) -> Self::T {
        self.0
    }
}

impl<T: Clone + Ord + Bounded> From<T> for SMax<T> {
    fn from(value: T) -> Self {
        Self(value)
    }
}

#[derive(Debug, Clone, Copy)]
pub struct SMin<T>(T);

impl<T: Clone + Ord + Bounded> SemiGroup for SMin<T> {
    type T = T;

    fn binary_operation(x: Self, y: Self) -> Self {
        x.get().min(y.get()).into()
    }

    fn get(self) -> Self::T {
        self.0
    }
}

impl<T: Clone + Ord + Bounded> From<T> for SMin<T> {
    fn from(value: T) -> Self {
        Self(value)
    }
}

#[derive(Debug, Clone, Copy)]
pub struct SSum<T: Add + Zero>(T);

impl<T: Clone + Add + Zero> SemiGroup for SSum<T> {
    type T = T;

    fn binary_operation(x: Self, y: Self) -> Self {
        (x.get() + y.get()).into()
    }

    fn get(self) -> Self::T {
        self.0
    }
}

impl<T: Clone + Add + Zero> From<T> for SSum<T> {
    fn from(value: T) -> Self {
        Self(value)
    }
}

#[derive(Debug, Clone, Copy)]
pub struct SProduct<T: Mul + One>(T);

impl<T: Clone + Mul + One> SemiGroup for SProduct<T> {
    type T = T;

    fn binary_operation(x: Self, y: Self) -> Self {
        (x.get() * y.get()).into()
    }

    fn get(self) -> Self::T {
        self.0
    }
}

impl<T: Clone + Mul + One> From<T> for SProduct<T> {
    fn from(value: T) -> Self {
        Self(value)
    }
}

#[derive(Debug, Clone, Copy)]
pub struct SBitAnd<T: BitAnd<Output = T> + Not<Output = T> + Zero>(T);

impl<T: Clone + BitAnd<Output = T> + Not<Output = T> + Zero> SemiGroup for SBitAnd<T> {
    type T = T;

    fn binary_operation(x: Self, y: Self) -> Self {
        (x.get() & y.get()).into()
    }

    fn get(self) -> Self::T {
        self.0
    }
}

impl<T: Clone + BitAnd<Output = T> + Not<Output = T> + Zero> From<T> for SBitAnd<T> {
    fn from(value: T) -> Self {
        Self(value)
    }
}

#[derive(Debug, Clone, Copy)]
pub struct SBitOr<T: BitOr<Output = T> + Zero>(T);

impl<T: Clone + BitOr<Output = T> + Zero> SemiGroup for SBitOr<T> {
    type T = T;

    fn binary_operation(x: Self, y: Self) -> Self {
        (x.get() | y.get()).into()
    }

    fn get(self) -> Self::T {
        self.0
    }
}

impl<T: Clone + BitOr<Output = T> + Zero> From<T> for SBitOr<T> {
    fn from(value: T) -> Self {
        Self(value)
    }
}

#[derive(Debug, Clone, Copy)]
pub struct SBitXor<T: BitXor<Output = T> + Zero>(T);

impl<T: Clone + BitXor<Output = T> + Zero> SemiGroup for SBitXor<T> {
    type T = T;

    fn binary_operation(x: Self, y: Self) -> Self {
        (x.get() ^ y.get()).into()
    }

    fn get(self) -> Self::T {
        self.0
    }
}

impl<T: Clone + BitXor<Output = T> + Zero> From<T> for SBitXor<T> {
    fn from(value: T) -> Self {
        Self(value)
    }
}

#[derive(Debug, Clone, Copy)]
pub struct SGcd<T>(T);

impl<T: Clone + Integer + Zero> SemiGroup for SGcd<T> {
    type T = T;

    fn binary_operation(x: Self, y: Self) -> Self {
        x.get().gcd(&y.get()).into()
    }

    fn get(self) -> Self::T {
        self.0
    }
}

impl<T: Clone + Integer + Zero> From<T> for SGcd<T> {
    fn from(value: T) -> Self {
        Self(value)
    }
}

#[derive(Debug, Clone, Copy)]
pub struct SLcm<T: Integer + One>(T);

impl<T: Clone + Integer + One> SemiGroup for SLcm<T> {
    type T = T;

    fn binary_operation(x: Self, y: Self) -> Self {
        x.get().lcm(&y.get()).into()
    }

    fn get(self) -> Self::T {
        self.0
    }
}

impl<T: Clone + Integer + One> From<T> for SLcm<T> {
    fn from(value: T) -> Self {
        Self(value)
    }
}

#[derive(Debug, Clone, Copy)]
pub struct SSingleton;

impl SemiGroup for SSingleton {
    type T = ();

    fn binary_operation(_: Self, _: Self) -> Self {
        Self
    }

    fn get(self) -> Self::T {
        ()
    }
}

impl From<()> for SSingleton {
    fn from(_: ()) -> Self {
        Self
    }
}
