use std::{
    convert::TryInto,
    fmt::{Debug, Display, Formatter},
    marker::PhantomData,
    num::{NonZeroU64, ParseIntError},
    str::FromStr,
};

pub trait Modulus: Copy {
    const VALUE: NonZeroU64;
    const IS_PRIME: bool;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Mod998244353 {}

impl Modulus for Mod998244353 {
    const VALUE: NonZeroU64 = unsafe { NonZeroU64::new_unchecked(998244353) };
    const IS_PRIME: bool = true;
}

pub type ModInt998244353 = ModInt<Mod998244353>;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Mod1000000007 {}

impl Modulus for Mod1000000007 {
    const VALUE: NonZeroU64 = unsafe { NonZeroU64::new_unchecked(1000000007) };
    const IS_PRIME: bool = true;
}

pub type ModInt1000000007 = ModInt<Mod1000000007>;

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct ModInt<O> {
    value: u64,
    phantom: PhantomData<fn() -> O>,
}

impl<O> ModInt<O> {
    pub fn get(&self) -> u64 {
        self.value
    }
}

impl<O: Modulus> ModInt<O> {
    pub fn order(&self) -> u64 {
        O::VALUE.get()
    }

    pub fn new<T>(value: T) -> Self
    where
        T: TryInto<i64>,
    {
        let value = value.try_into().unwrap_or_else(|_| {
            panic!(
                "failed to convet integer type: {} -> i64",
                std::any::type_name::<T>()
            )
        });

        let value = if value > O::VALUE.get() as i64 {
            value as u64 % O::VALUE.get()
        } else if value >= 0 {
            value as u64
        } else {
            O::VALUE.get() - (value.abs() as u64 % O::VALUE.get())
        };

        Self {
            value,
            phantom: PhantomData,
        }
    }
}

macro_rules! impl_from_primitive {
    ($($t:ty),*) => {
        $(
            impl<O: Modulus> From<$t> for ModInt<O> {
                fn from(value: $t) -> Self {
                    Self::new(value)
                }
            }
        )*
    };
}

impl_from_primitive!(u8, u16, u32, u64, usize, i8, i16, i32, i64, isize);

impl<O: Modulus> FromStr for ModInt<O> {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let value: i64 = s.parse()?;
        Ok(Self::new(value))
    }
}

impl<O: Modulus> Debug for ModInt<O> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} (mod. {})", self.get(), O::VALUE)
    }
}

impl<O: Modulus> Display for ModInt<O> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        Display::fmt(&self.get(), f)
    }
}

use num_traits::{Inv, One, Pow, Zero};
use std::{
    iter::{Product, Sum},
    ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign},
};

impl<O: Modulus> Add for ModInt<O> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self::new(if self.get() + rhs.get() < O::VALUE.get() {
            self.get() + rhs.get()
        } else {
            self.get() + rhs.get() - O::VALUE.get()
        })
    }
}

impl<O: Modulus> AddAssign for ModInt<O> {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}

impl<O: Modulus> Sub for ModInt<O> {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self::new(if self.get() >= rhs.get() {
            self.get() - rhs.get()
        } else {
            self.get() + O::VALUE.get() - rhs.get()
        })
    }
}

impl<O: Modulus> SubAssign for ModInt<O> {
    fn sub_assign(&mut self, rhs: Self) {
        *self = *self - rhs;
    }
}

impl<O: Modulus> Mul for ModInt<O> {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        let value = (self.get() * rhs.get()) % O::VALUE.get();
        Self::new(value)
    }
}

impl<O: Modulus> MulAssign for ModInt<O> {
    fn mul_assign(&mut self, rhs: Self) {
        *self = *self * rhs;
    }
}

impl<O: Modulus> Div for ModInt<O> {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        self * rhs.inv()
    }
}

impl<O: Modulus> DivAssign for ModInt<O> {
    fn div_assign(&mut self, rhs: Self) {
        *self = *self / rhs
    }
}

impl<O: Modulus> Sum for ModInt<O> {
    fn sum<I>(iter: I) -> Self
    where
        I: Iterator<Item = Self>,
    {
        iter.fold(Self::zero(), |acc, x| acc + x)
    }
}

impl<O: Modulus> Product for ModInt<O> {
    fn product<I>(iter: I) -> Self
    where
        I: Iterator<Item = Self>,
    {
        iter.fold(Self::one(), |acc, x| acc * x)
    }
}

macro_rules! impl_ops_for_unsigned_int {
    ($($t:ty),*) => {
        $(
            impl<O: Modulus> Add<$t> for ModInt<O> {
                type Output = Self;

                fn add(self, rhs: $t) -> Self::Output {
                    let value = self.get() + rhs as u64;
                    Self::new(if value < O::VALUE.get() {
                        value
                    } else {
                        value - O::VALUE.get()
                    })
                }
            }

            impl<O: Modulus> Add<ModInt<O>> for $t {
                type Output = ModInt<O>;

                fn add(self, rhs: ModInt<O>) -> Self::Output {
                    rhs + self
                }
            }

            impl<O: Modulus> AddAssign<$t> for ModInt<O> {
                fn add_assign(&mut self, rhs: $t) {
                    *self = *self + rhs;
                }
            }

            impl<O: Modulus> Sub<$t> for ModInt<O> {
                type Output = Self;

                fn sub(self, rhs: $t) -> Self::Output {
                    let rhs = rhs as u64;
                    if self.get() >= rhs {
                        Self::new(self.get() - rhs)
                    } else {
                        self + (-Self::new(rhs))
                    }
                }
            }

            impl<O: Modulus> Sub<ModInt<O>> for $t {
                type Output = ModInt<O>;

                fn sub(self, rhs: ModInt<O>) -> Self::Output {
                    -(rhs - self)
                }
            }

            impl<O: Modulus> SubAssign<$t> for ModInt<O> {
                fn sub_assign(&mut self, rhs: $t) {
                    *self = *self - rhs;
                }
            }

            impl<O: Modulus> Mul<$t> for ModInt<O> {
                type Output = Self;

                fn mul(self, rhs: $t) -> Self::Output {
                    let value = self.get() * rhs as u64 % O::VALUE.get();
                    Self::new(value)
                }
            }

            impl<O: Modulus> Mul<ModInt<O>> for $t {
                type Output = ModInt<O>;

                fn mul(self, rhs: ModInt<O>) -> Self::Output {
                    rhs * self
                }
            }

            impl<O: Modulus> MulAssign<$t> for ModInt<O> {
                fn mul_assign(&mut self, rhs: $t) {
                    *self = *self * rhs;
                }
            }

            impl<O: Modulus> Div<$t> for ModInt<O> {
                type Output = Self;

                fn div(self, rhs: $t) -> Self::Output {
                    self * Self::new(rhs).inv()
                }
            }

            impl<O: Modulus> Div<ModInt<O>> for $t {
                type Output = ModInt<O>;

                fn div(self, rhs: ModInt<O>) -> Self::Output {
                    (rhs / self).inv()
                }
            }


            impl<O: Modulus> DivAssign<$t> for ModInt<O> {
                fn div_assign(&mut self, rhs: $t) {
                    *self = *self / rhs
                }
            }

            impl<O: Modulus> Sum<$t> for ModInt<O> {
                fn sum<I>(iter: I) -> Self
                where
                    I: Iterator<Item = $t>,
                {
                    iter.fold(Self::zero(), |acc, x| acc + x)
                }
            }

            impl<O: Modulus> Product<$t> for ModInt<O> {
                fn product<I>(iter: I) -> Self
                where
                    I: Iterator<Item = $t>,
                {
                    iter.fold(Self::one(), |acc, x| acc * x)
                }
            }

            impl<O: Modulus> Pow<$t> for ModInt<O> {
                type Output = Self;

                fn pow(self, mut exp: $t) -> Self::Output {
                    let mut res = ModInt::one();
                    let mut cur = self;
                    while exp != 0 {
                        if exp & 1 != 0 {
                            res *= cur;
                        }
                        cur *= cur;
                        exp >>= 1;
                    }
                    res
                }
            }
        )*
    };
}

impl_ops_for_unsigned_int!(u8, u16, u32, u64, usize);

macro_rules! impl_ops_for_signed_int {
    ($($t:ty),*) => {
        $(
            impl<O: Modulus> Add<$t> for ModInt<O> {
                type Output = Self;

                fn add(self, rhs: $t) -> Self::Output {
                    self + Self::new(rhs)
                }
            }

            impl<O: Modulus> AddAssign<$t> for ModInt<O> {
                fn add_assign(&mut self, rhs: $t) {
                    *self = *self + rhs;
                }
            }


            impl<O: Modulus> Sub<$t> for ModInt<O> {
                type Output = Self;

                fn sub(self, rhs: $t) -> Self::Output {
                    self - Self::new(rhs)
                }
            }

            impl<O: Modulus> SubAssign<$t> for ModInt<O> {
                fn sub_assign(&mut self, rhs: $t) {
                    *self = *self - rhs;
                }
            }

            impl<O: Modulus> Mul<$t> for ModInt<O> {
                type Output = Self;

                fn mul(self, rhs: $t) -> Self::Output {
                    self * Self::new(rhs)
                }
            }

            impl<O: Modulus> MulAssign<$t> for ModInt<O> {
                fn mul_assign(&mut self, rhs: $t) {
                    *self = *self * rhs;
                }
            }

            impl<O: Modulus> Div<$t> for ModInt<O> {
                type Output = Self;

                fn div(self, rhs: $t) -> Self::Output {
                    self * Self::new(rhs).inv()
                }
            }

            impl<O: Modulus> DivAssign<$t> for ModInt<O> {
                fn div_assign(&mut self, rhs: $t) {
                    *self = *self / rhs
                }
            }

            impl<O: Modulus> Sum<$t> for ModInt<O> {
                fn sum<I>(iter: I) -> Self
                where
                    I: Iterator<Item = $t>,
                {
                    iter.fold(Self::zero(), |acc, x| acc + x)
                }
            }

            impl<O: Modulus> Product<$t> for ModInt<O> {
                fn product<I>(iter: I) -> Self
                where
                    I: Iterator<Item = $t>,
                {
                    iter.fold(Self::one(), |acc, x| acc * x)
                }
            }
        )*
    };
}

impl_ops_for_signed_int!(i8, i16, i32, i64, isize);

impl<O: Modulus> Neg for ModInt<O> {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self::zero() - self
    }
}

impl<O: Modulus> Zero for ModInt<O> {
    fn zero() -> Self {
        Self::new(0)
    }

    fn is_zero(&self) -> bool {
        self.get() == 0
    }
}

impl<O: Modulus> One for ModInt<O> {
    fn one() -> Self {
        Self::new(1)
    }

    fn is_one(&self) -> bool {
        self.get() == 1
    }
}

impl<O: Modulus> Inv for ModInt<O> {
    type Output = Self;

    fn inv(self) -> Self::Output {
        if self.get() == 0 {
            panic!("attempt to divide by zero")
        } else if O::IS_PRIME {
            self.pow(O::VALUE.get() - 2)
        } else {
            unimplemented!("division at the non-prime order")
        }
    }
}

impl<O> vis::visualize::Visualize for ModInt<O> {
    fn visualize(&self, _split: &str) -> String {
        self.get().visualize(_split)
    }
}
