use super::semigroup::{
    SBitAnd, SBitOr, SBitXor, SGcd, SLcm, SMax, SMin, SProduct, SSingleton, SSum, SemiGroup,
};
use num_integer::Integer;
use num_traits::{Bounded, One, Zero};
use std::ops::{Add, BitAnd, BitOr, BitXor, Mul, Not};

pub trait Monoid: SemiGroup {
    fn identity() -> Self;
}

impl<T: Clone + Ord + Bounded> Monoid for SMax<T> {
    fn identity() -> Self {
        T::min_value().into()
    }
}

pub type MMax<T> = SMax<T>;

impl<T: Clone + Ord + Bounded> Monoid for SMin<T> {
    fn identity() -> Self {
        T::max_value().into()
    }
}

pub type MMin<T> = SMin<T>;

impl<T: Clone + Add + Zero> Monoid for SSum<T> {
    fn identity() -> Self {
        T::zero().into()
    }
}

pub type MSum<T> = SSum<T>;

impl<T: Clone + Mul + One> Monoid for SProduct<T> {
    fn identity() -> Self {
        T::one().into()
    }
}

pub type MProduct<T> = SProduct<T>;

impl<T: Clone + BitAnd<Output = T> + Not<Output = T> + Zero> Monoid for SBitAnd<T> {
    fn identity() -> Self {
        (!T::zero()).into()
    }
}

pub type MBitAnd<T> = SBitAnd<T>;

impl<T: Clone + BitOr<Output = T> + Zero> Monoid for SBitOr<T> {
    fn identity() -> Self {
        T::zero().into()
    }
}

pub type MBitOr<T> = SBitOr<T>;

impl<T: Clone + BitXor<Output = T> + Zero> Monoid for SBitXor<T> {
    fn identity() -> Self {
        T::zero().into()
    }
}

pub type MBitXor<T> = SBitXor<T>;

impl<T: Clone + Integer + Zero> Monoid for SGcd<T> {
    fn identity() -> Self {
        T::zero().into()
    }
}

pub type MGcd<T> = SGcd<T>;

impl<T: Clone + Integer + One> Monoid for SLcm<T> {
    fn identity() -> Self {
        T::one().into()
    }
}

pub type MLcm<T> = SLcm<T>;

impl Monoid for SSingleton {
    fn identity() -> Self {
        Self
    }
}

pub type MSingleton = SSingleton;

// #[macro_export]
// macro_rules! def_monoid {
//     ($name:ident<$kind:ty>, $identity:expr, $op:expr) => {
//         #[derive(Debug, Clone)]
//         pub struct $name($kind);
//         impl Monoid for $name {
//             type T = $kind;
//             fn identity() -> Self {
//                 Self($identity)
//             }
//         }
//     };
// }
// def_monoid!(MLogAnd<bool>, true, |x, y| x && y);
// def_monoid!(MLogOr<bool>, false, |x, y| x || y);
// def_monoid!(MLogXor<bool>, false, |x, y| x ^ y);
