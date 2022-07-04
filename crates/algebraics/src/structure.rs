use num_integer::Integer;
use num_traits::{Bounded, One, Zero};

use crate::{
    abstract_type::{AbelianGroup, Monoid},
    property::{
        Associativity, Cancellativity, Commutativity, Idempotent, Identity, Invertibility,
        Operation,
    },
};
use std::{
    cmp::{max, min},
    ops::{Add, Mul, Neg},
};

// impl<T: Clone + Operation<T>> Operation<Option<T>> for Option<T> {
//     fn operate(x: Option<T>, y: Option<T>) -> Option<T> {
//         match (x, y) {
//             (None, None) => None,
//             (None, Some(y)) => Some(y),
//             (Some(x), None) => Some(x),
//             (Some(x), Some(y)) => Some(T::operate(x, y)),
//         }
//     }
// }

// impl<T: Clone + PartialEq + Operation<T>> Identity<Option<T>> for Option<T> {
//     fn identity() -> Option<T> {
//         None
//     }
// }

// impl<T: Clone + PartialEq + Operation<T>> Monoid for Option<T> {
//     type I = T;

//     fn get(self) -> Self::I {
//         todo!()
//     }
// }

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Min<I>(pub I);

impl<I> From<I> for Min<I> {
    fn from(x: I) -> Self {
        Self(x)
    }
}

impl<I: Clone + Ord> Operation<I> for Min<I> {
    fn operate(x: I, y: I) -> I {
        min(x, y)
    }
}

impl<I: Clone + Ord + Bounded> Identity<I> for Min<I> {
    fn identity() -> I {
        I::max_value()
    }
}

impl<I: Clone + Ord> Associativity<I> for Min<I> {}

impl<I: Clone + Ord + Bounded> Monoid for Min<I> {
    type I = I;

    fn get(self) -> I {
        self.0
    }
}

impl<I: Clone + Ord> Idempotent<I> for Min<I> {}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Max<I>(pub I);

impl<I> From<I> for Max<I> {
    fn from(x: I) -> Self {
        Self(x)
    }
}

impl<I: Clone + Ord> Operation<I> for Max<I> {
    fn operate(x: I, y: I) -> I {
        max(x, y)
    }
}

impl<I: Clone + Ord + Bounded> Identity<I> for Max<I> {
    fn identity() -> I {
        I::min_value()
    }
}

impl<I: Clone + Ord> Associativity<I> for Max<I> {}

impl<I: Clone + Ord + Bounded> Monoid for Max<I> {
    type I = I;

    fn get(self) -> I {
        self.0
    }
}

impl<I: Clone + Ord> Idempotent<I> for Max<I> {}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Additive<I>(pub I);

impl<I> From<I> for Additive<I> {
    fn from(x: I) -> Self {
        Self(x)
    }
}

impl<I: Clone + Add<Output = I>> Operation<I> for Additive<I> {
    fn operate(x: I, y: I) -> I {
        x + y
    }
}

impl<I: Clone + PartialEq + Add<Output = I> + Zero> Identity<I> for Additive<I> {
    fn identity() -> I {
        I::zero()
    }
}

impl<I: Clone + PartialEq + Add<Output = I>> Associativity<I> for Additive<I> {}

macro_rules! impl_monoid_for_additive_unsigned_int {
    ($($t:ty),*) => {
        $(
            impl Monoid for Additive<$t> {
                type I = $t;

                fn get(self) -> Self::I {
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
            impl Invertibility<$t> for Additive<$t> {
                fn inverse(x: $t) -> $t {
                    x.neg()
                }
            }

            impl Cancellativity<$t> for Additive<$t> {}

            impl Commutativity<$t> for Additive<$t> {}

            impl AbelianGroup for Additive<$t> {
                type I = $t;

                fn get(self) -> $t {
                    self.0
                }
            }
        )*
    };
}

impl_abelian_group_for_additive_signed_int!(isize, i8, i16, i32, i64, i128);

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Multiplicative<I>(pub I);

impl<I> From<I> for Multiplicative<I> {
    fn from(x: I) -> Self {
        Self(x)
    }
}

impl<I: Clone + Mul<Output = I>> Operation<I> for Multiplicative<I> {
    fn operate(x: I, y: I) -> I {
        x * y
    }
}

impl<I: Clone + PartialEq + Mul<Output = I> + One> Identity<I> for Multiplicative<I> {
    fn identity() -> I {
        I::one()
    }
}

impl<I: Clone + PartialEq + Mul<Output = I>> Associativity<I> for Multiplicative<I> {}

impl<I: Clone + PartialEq + Mul<Output = I> + One> Monoid for Multiplicative<I> {
    type I = I;

    fn get(self) -> I {
        self.0
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Gcd<I>(pub I);

impl<I> From<I> for Gcd<I> {
    fn from(x: I) -> Self {
        Self(x)
    }
}

impl<I: Clone + Integer> Operation<I> for Gcd<I> {
    fn operate(x: I, y: I) -> I {
        x.gcd(&y)
    }
}

impl<I: Clone + PartialEq + Integer + Zero> Identity<I> for Gcd<I> {
    fn identity() -> I {
        I::zero()
    }
}

impl<I: Clone + PartialEq + Integer> Associativity<I> for Gcd<I> {}

impl<I: Clone + PartialEq + Integer> Monoid for Gcd<I> {
    type I = I;

    fn get(self) -> I {
        self.0
    }
}

impl<I: Clone + PartialEq + Integer> Idempotent<I> for Gcd<I> {}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Lcm<I>(pub I);

impl<I> From<I> for Lcm<I> {
    fn from(x: I) -> Self {
        Self(x)
    }
}

impl<I: Clone + Integer> Operation<I> for Lcm<I> {
    fn operate(x: I, y: I) -> I {
        x.lcm(&y)
    }
}

impl<I: Clone + PartialEq + Integer + One> Identity<I> for Lcm<I> {
    fn identity() -> I {
        I::one()
    }
}

impl<I: Clone + PartialEq + Integer> Associativity<I> for Lcm<I> {}

impl<I: Clone + PartialEq + Integer> Monoid for Lcm<I> {
    type I = I;

    fn get(self) -> I {
        self.0
    }
}

impl<I: Clone + PartialEq + Integer> Idempotent<I> for Lcm<I> {}

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

        impl $crate::property::Operation<$t> for $name {
            fn operate(x: $t, y: $t) -> $t {
                let f = $operation;
                f(x, y)
            }
        }

        impl $crate::abstract_type::Magma for $name {
            type I = $t;

            fn get(self) -> Self::I {
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

        impl $crate::property::Operation<$t> for $name {
            fn operate(x: $t, y: $t) -> $t {
                let f = $operation;
                f(x, y)
            }
        }

        impl $crate::property::Associativity<$t> for $name {}

        impl $crate::abstract_type::SemiGroup for $name {
            type I = $t;

            fn get(self) -> Self::I {
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
        Additive::check_associative(a, b, c);
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

        impl $crate::property::Operation<$t> for $name {
            fn operate(x: $t, y: $t) -> $t {
                let f = $operation;
                f(x, y)
            }
        }

        impl $crate::property::Identity<$t> for $name {
            fn identity() -> $t {
                $identity.into()
            }
        }

        impl $crate::property::Associativity<$t> for $name {}

        impl $crate::abstract_type::Monoid for $name {
            type I = $t;

            fn get(self) -> $t {
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
        Additive::check_associative(a, b, c);
        Additive::check_identity(a);
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

        impl $crate::property::Operation<$t> for $name {
            fn operate(x: $t, y: $t) -> $t {
                let f = $operation;
                f(x, y)
            }
        }

        impl $crate::property::Identity<$t> for $name {
            fn identity() -> $t {
                $identity
            }
        }

        impl $crate::property::Invertibility<$t> for $name {
            fn inverse(x: $t) -> $t {
                let f = $inverse;
                f(x)
            }
        }

        impl $crate::property::Associativity<$t> for $name {}

        impl $crate::property::Cancellativity<$t> for $name {}

        impl $crate::abstract_type::Group for $name {
            type I = $t;

            fn get(self) -> $t {
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
        Additive::check_associative(a, b, c);
        Additive::check_cancellativity(a, b, c);
        Additive::check_identity(a);
        Additive::check_invertibility(a);
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

        impl $crate::property::Operation<$t> for $name {
            fn operate(x: $t, y: $t) -> $t {
                let f = $operation;
                f(x, y)
            }
        }

        impl $crate::property::Identity<$t> for $name {
            fn identity() -> $t {
                $identity
            }
        }

        impl $crate::property::Invertibility<$t> for $name {
            fn inverse(x: $t) -> $t {
                let f = $inverse;
                f(x)
            }
        }

        impl $crate::property::Associativity<$t> for $name {}

        impl $crate::property::Cancellativity<$t> for $name {}

        impl $crate::property::Commutativity<$t> for $name {}

        impl $crate::abstract_type::AbelianGroup for $name {
            type I = $t;

            fn get(self) -> $t {
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
        Additive::check_associative(a, b, c);
        Additive::check_cancellativity(a, b, c);
        Additive::check_commutative(a, b);
        Additive::check_identity(a);
        Additive::check_invertibility(a);
    }
}
