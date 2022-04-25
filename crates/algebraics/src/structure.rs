use num_integer::Integer;
use num_traits::{Bounded, One, Zero};

use crate::{
    abstruct::{AbelianGroup, Monoid},
    property::{Associativity, Cancellativity, Commutativity, Idempotent, Identity, Invertibility},
    Operation,
};
use std::{
    cmp::{max, min},
    ops::{Add, Mul, Neg},
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Min<T>(pub T);

impl<T> From<T> for Min<T> {
    fn from(x: T) -> Self {
        Self(x)
    }
}

impl<T: Clone + Ord> Operation for Min<T> {
    fn operate(self, rhs: Self) -> Self {
        min(self, rhs)
    }
}

impl<T: Clone + Ord + Bounded> Identity for Min<T> {
    fn identity() -> Self {
        T::max_value().into()
    }
}

impl<T: Clone + Ord> Associativity for Min<T> {}

impl<T: Clone + Ord + Bounded> Monoid for Min<T> {
    type T = T;

    fn get(self) -> Self::T {
        self.0
    }
}

impl<T: Clone + Ord> Idempotent for Min<T> {}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Max<T>(pub T);

impl<T> From<T> for Max<T> {
    fn from(x: T) -> Self {
        Self(x)
    }
}

impl<T: Clone + Ord> Operation for Max<T> {
    fn operate(self, rhs: Self) -> Self {
        max(self, rhs)
    }
}

impl<T: Clone + Ord + Bounded> Identity for Max<T> {
    fn identity() -> Self {
        T::min_value().into()
    }
}

impl<T: Clone + Ord> Associativity for Max<T> {}

impl<T: Clone + Ord + Bounded> Monoid for Max<T> {
    type T = T;

    fn get(self) -> Self::T {
        self.0
    }
}

impl<T: Clone + Ord> Idempotent for Max<T> {}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Additive<T>(pub T);

impl<T> From<T> for Additive<T> {
    fn from(x: T) -> Self {
        Self(x)
    }
}

impl<T: Clone + Add<Output = T>> Operation for Additive<T> {
    fn operate(self, rhs: Self) -> Self {
        (self.0 + rhs.0).into()
    }
}

impl<T: Clone + PartialEq + Add<Output = T> + Zero> Identity for Additive<T> {
    fn identity() -> Self {
        T::zero().into()
    }
}

impl<T: Clone + PartialEq + Add<Output = T>> Associativity for Additive<T> {}

macro_rules! impl_monoid_for_additive_unsigned_int {
    ($($t:ty),*) => {
        $(
            impl Monoid for Additive<$t> {
                type T = $t;

                fn get(self) -> Self::T {
                    self.0
                }
            }
        )*
    };
}

impl_monoid_for_additive_unsigned_int!(usize, u8, u16, u32, u64, u128);

macro_rules! impl_abelian_group_for_additive_signed_int {
    ($($t:ty),*) => {
        $(
            impl Invertibility for Additive<$t> {
                fn inverse(self) -> Self {
                    self.0.neg().into()
                }
            }

            impl Cancellativity for Additive<$t> {}

            impl Commutativity for Additive<$t> {}

            impl AbelianGroup for Additive<$t> {
                type T = $t;

                fn get(self) -> Self::T {
                    self.0
                }
            }
        )*
    };
}

impl_abelian_group_for_additive_signed_int!(isize, i8, i16, i32, i64, i128);

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Multiplicative<T>(pub T);

impl<T> From<T> for Multiplicative<T> {
    fn from(x: T) -> Self {
        Self(x)
    }
}

impl<T: Clone + Mul<Output = T>> Operation for Multiplicative<T> {
    fn operate(self, rhs: Self) -> Self {
        (self.0 * rhs.0).into()
    }
}

impl<T: Clone + PartialEq + Mul<Output = T> + One> Identity for Multiplicative<T> {
    fn identity() -> Self {
        T::one().into()
    }
}

impl<T: Clone + PartialEq + Mul<Output = T>> Associativity for Multiplicative<T> {}

impl<T: Clone + PartialEq + Mul<Output = T> + One> Monoid for Multiplicative<T> {
    type T = T;

    fn get(self) -> Self::T {
        self.0
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Gcd<T>(pub T);

impl<T> From<T> for Gcd<T> {
    fn from(x: T) -> Self {
        Self(x)
    }
}

impl<T: Clone + Integer> Operation for Gcd<T> {
    fn operate(self, rhs: Self) -> Self {
        self.0.gcd(&rhs.0).into()
    }
}

impl<T: Clone + PartialEq + Integer + Zero> Identity for Gcd<T> {
    fn identity() -> Self {
        T::zero().into()
    }
}

impl<T: Clone + PartialEq + Integer> Associativity for Gcd<T> {}

impl<T: Clone + PartialEq + Integer> Monoid for Gcd<T> {
    type T = T;

    fn get(self) -> Self::T {
        self.0
    }
}

impl<T: Clone + PartialEq + Integer> Idempotent for Gcd<T> {}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Lcm<T>(pub T);

impl<T> From<T> for Lcm<T> {
    fn from(x: T) -> Self {
        Self(x)
    }
}

impl<T: Clone + Integer> Operation for Lcm<T> {
    fn operate(self, rhs: Self) -> Self {
        self.0.lcm(&rhs.0).into()
    }
}

impl<T: Clone + PartialEq + Integer + One> Identity for Lcm<T> {
    fn identity() -> Self {
        T::one().into()
    }
}

impl<T: Clone + PartialEq + Integer> Associativity for Lcm<T> {}

impl<T: Clone + PartialEq + Integer> Monoid for Lcm<T> {
    type T = T;

    fn get(self) -> Self::T {
        self.0
    }
}

impl<T: Clone + PartialEq + Integer> Idempotent for Lcm<T> {}

#[macro_export]
macro_rules! impl_magma {
    ($name:tt($t:ty), $operation:expr) => {
        #[derive(Debug, Clone, PartialEq, Hash)]
        pub struct $name(pub $t);

        impl From<$t> for $name {
            fn from(x: $t) -> Self {
                Self(x)
            }
        }

        impl $crate::Operation for $name {
            fn operate(self, rhs: Self) -> Self {
                let f = $operation;
                f(self.0, rhs.0).into()
            }
        }

        impl $crate::abstruct::Magma for $name {
            type T = $t;

            fn get(self) -> Self::T {
                self.0
            }
        }
    };
}

#[test]
fn impl_magma_test() {
    impl_magma!(Additive(i64), |x, y| x + y);
}

#[macro_export]
macro_rules! impl_semigroup {
    ($name:tt($t:ty), $operation:expr) => {
        #[derive(Debug, Clone, PartialEq, Hash)]
        pub struct $name(pub $t);

        impl From<$t> for $name {
            fn from(x: $t) -> Self {
                Self(x)
            }
        }

        impl $crate::Operation for $name {
            fn operate(self, rhs: Self) -> Self {
                let f = $operation;
                f(self.0, rhs.0).into()
            }
        }

        impl $crate::property::Associativity for $name {}

        impl $crate::abstruct::SemiGroup for $name {
            type T = $t;

            fn get(self) -> Self::T {
                self.0
            }
        }
    };
}

#[test]
fn impl_semigroup_test() {
    impl_semigroup!(Additive(i64), |x, y| x + y);

    use rand::Rng;
    let mut rng = rand::thread_rng();
    for _ in 0..1000 {
        let a: i64 = rng.gen_range(-1000..=1000);
        let b: i64 = rng.gen_range(-1000..=1000);
        let c: i64 = rng.gen_range(-1000..=1000);
        dbg!(a, b, c);
        Additive::check_associative(a.into(), b.into(), c.into());
    }
}

#[macro_export]
macro_rules! impl_monoid {
    ($name:tt($t:ty), $operation:expr, $identity:expr) => {
        #[derive(Debug, Clone, PartialEq, Hash)]
        pub struct $name(pub $t);

        impl From<$t> for $name {
            fn from(x: $t) -> Self {
                Self(x)
            }
        }

        impl $crate::Operation for $name {
            fn operate(self, rhs: Self) -> Self {
                let f = $operation;
                f(self.0, rhs.0).into()
            }
        }

        impl $crate::property::Identity for $name {
            fn identity() -> Self {
                $identity.into()
            }
        }

        impl $crate::property::Associativity for $name {}

        impl $crate::abstruct::Monoid for $name {
            type T = $t;

            fn get(self) -> Self::T {
                self.0
            }
        }
    };
}

#[test]
fn impl_monoid_test() {
    impl_monoid!(Additive(i64), |x, y| x + y, 0);

    use rand::Rng;
    let mut rng = rand::thread_rng();
    for _ in 0..1000 {
        let a: i64 = rng.gen_range(-1000..=1000);
        let b: i64 = rng.gen_range(-1000..=1000);
        let c: i64 = rng.gen_range(-1000..=1000);
        dbg!(a, b, c);
        Additive::check_associative(a.into(), b.into(), c.into());
        Additive::check_identity(a.into());
    }
}

#[macro_export]
macro_rules! impl_group {
    ($name:tt($t:ty), $operation:expr, $identity:expr, $inverse:expr) => {
        #[derive(Debug, Clone, PartialEq, Hash)]
        pub struct $name(pub $t);

        impl From<$t> for $name {
            fn from(x: $t) -> Self {
                Self(x)
            }
        }

        impl $crate::Operation for $name {
            fn operate(self, rhs: Self) -> Self {
                let f = $operation;
                f(self.0, rhs.0).into()
            }
        }

        impl $crate::property::Identity for $name {
            fn identity() -> Self {
                $identity.into()
            }
        }

        impl $crate::property::Invertibility for $name {
            fn inverse(self) -> Self {
                let f = $inverse;
                f(self.0).into()
            }
        }

        impl $crate::property::Associativity for $name {}

        impl $crate::property::Cancellativity for $name {}

        impl $crate::abstruct::Group for $name {
            type T = $t;

            fn get(self) -> Self::T {
                self.0
            }
        }
    };
}

#[test]
fn impl_group_test() {
    impl_group!(Additive(i64), |x, y| x + y, 0, |x: i64| -x);

    use rand::Rng;
    let mut rng = rand::thread_rng();
    for _ in 0..1000 {
        let a: i64 = rng.gen_range(-1000..=1000);
        let b: i64 = rng.gen_range(-1000..=1000);
        let c: i64 = rng.gen_range(-1000..=1000);
        dbg!(a, b, c);
        Additive::check_associative(a.into(), b.into(), c.into());
        Additive::check_cancellativity(a.into(), b.into(), c.into());
        Additive::check_identity(a.into());
        Additive::check_invertibility(a.into());
    }
}

#[macro_export]
macro_rules! impl_abelian_group {
    ($name:tt($t:ty), $operation:expr, $identity:expr, $inverse:expr) => {
        #[derive(Debug, Clone, PartialEq, Hash)]
        pub struct $name(pub $t);

        impl From<$t> for $name {
            fn from(x: $t) -> Self {
                Self(x)
            }
        }

        impl $crate::Operation for $name {
            fn operate(self, rhs: Self) -> Self {
                let f = $operation;
                f(self.0, rhs.0).into()
            }
        }

        impl $crate::property::Identity for $name {
            fn identity() -> Self {
                $identity.into()
            }
        }

        impl $crate::property::Invertibility for $name {
            fn inverse(self) -> Self {
                let f = $inverse;
                f(self.0).into()
            }
        }

        impl $crate::property::Associativity for $name {}

        impl $crate::property::Cancellativity for $name {}

        impl $crate::property::Commutativity for $name {}

        impl $crate::abstruct::AbelianGroup for $name {
            type T = $t;

            fn get(self) -> Self::T {
                self.0
            }
        }
    };
}

#[test]
fn impl_ablian_group_test() {
    impl_abelian_group!(Additive(i64), |x, y| x + y, 0, |x: i64| -x);

    use rand::Rng;
    let mut rng = rand::thread_rng();
    for _ in 0..1000 {
        let a: i64 = rng.gen_range(-1000..=1000);
        let b: i64 = rng.gen_range(-1000..=1000);
        let c: i64 = rng.gen_range(-1000..=1000);
        dbg!(a, b, c);
        Additive::check_associative(a.into(), b.into(), c.into());
        Additive::check_cancellativity(a.into(), b.into(), c.into());
        Additive::check_commutative(a.into(), b.into());
        Additive::check_identity(a.into());
        Additive::check_invertibility(a.into());
    }
}
