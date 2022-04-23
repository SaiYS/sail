use num_integer::Integer;
use num_traits::{Inv, One, Pow, Zero};
use std::{
    convert::TryInto,
    fmt::{Debug, Display, Formatter},
    iter::{Product, Sum},
    marker::PhantomData,
    num::{NonZeroU64, ParseIntError},
    ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign},
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
pub struct ModInt<M> {
    value: u64,
    phantom: PhantomData<fn() -> M>,
}

impl<M> ModInt<M> {
    pub fn get(&self) -> u64 {
        self.value
    }
}

impl<M: Modulus> ModInt<M> {
    pub fn order(&self) -> u64 {
        M::VALUE.get()
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

        let value = if value > M::VALUE.get() as i64 {
            value as u64 % M::VALUE.get()
        } else if value >= 0 {
            value as u64
        } else {
            M::VALUE.get() - (value.abs() as u64 % M::VALUE.get())
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
            impl<M: Modulus> From<$t> for ModInt<M> {
                fn from(value: $t) -> Self {
                    Self::new(value)
                }
            }
        )*
    };
}

impl_from_primitive!(u8, u16, u32, u64, usize, i8, i16, i32, i64, isize);

impl<M: Modulus> FromStr for ModInt<M> {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let value: i64 = s.parse()?;
        Ok(Self::new(value))
    }
}

impl<M: Modulus> Debug for ModInt<M> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} (mod. {})", self.get(), M::VALUE)
    }
}

impl<M: Modulus> Display for ModInt<M> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        Display::fmt(&self.get(), f)
    }
}

impl<M: Modulus> Add for ModInt<M> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self::new(if self.get() + rhs.get() < M::VALUE.get() {
            self.get() + rhs.get()
        } else {
            self.get() + rhs.get() - M::VALUE.get()
        })
    }
}

impl<M: Modulus> AddAssign for ModInt<M> {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}

impl<M: Modulus> Sub for ModInt<M> {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self::new(if self.get() >= rhs.get() {
            self.get() - rhs.get()
        } else {
            self.get() + M::VALUE.get() - rhs.get()
        })
    }
}

impl<M: Modulus> SubAssign for ModInt<M> {
    fn sub_assign(&mut self, rhs: Self) {
        *self = *self - rhs;
    }
}

impl<M: Modulus> Mul for ModInt<M> {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        let value = (self.get() * rhs.get()) % M::VALUE.get();
        Self::new(value)
    }
}

impl<M: Modulus> MulAssign for ModInt<M> {
    fn mul_assign(&mut self, rhs: Self) {
        *self = *self * rhs;
    }
}

impl<M: Modulus> Div for ModInt<M> {
    type Output = Self;

    #[allow(clippy::suspicious_arithmetic_impl)]
    fn div(self, rhs: Self) -> Self::Output {
        self * rhs.inv()
    }
}

impl<M: Modulus> DivAssign for ModInt<M> {
    fn div_assign(&mut self, rhs: Self) {
        *self = *self / rhs
    }
}

impl<M: Modulus> Sum for ModInt<M> {
    fn sum<I>(iter: I) -> Self
    where
        I: Iterator<Item = Self>,
    {
        iter.fold(Self::zero(), |acc, x| acc + x)
    }
}

impl<M: Modulus> Product for ModInt<M> {
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
            impl<M: Modulus> Add<$t> for ModInt<M> {
                type Output = Self;

                fn add(self, rhs: $t) -> Self::Output {
                    let value = self.get() + rhs as u64;
                    Self::new(if value < M::VALUE.get() {
                        value
                    } else {
                        value - M::VALUE.get()
                    })
                }
            }

            impl<M: Modulus> Add<ModInt<M>> for $t {
                type Output = ModInt<M>;

                fn add(self, rhs: ModInt<M>) -> Self::Output {
                    rhs + self
                }
            }

            impl<M: Modulus> AddAssign<$t> for ModInt<M> {
                fn add_assign(&mut self, rhs: $t) {
                    *self = *self + rhs;
                }
            }

            impl<M: Modulus> Sub<$t> for ModInt<M> {
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

            impl<M: Modulus> Sub<ModInt<M>> for $t {
                type Output = ModInt<M>;

                fn sub(self, rhs: ModInt<M>) -> Self::Output {
                    -(rhs - self)
                }
            }

            impl<M: Modulus> SubAssign<$t> for ModInt<M> {
                fn sub_assign(&mut self, rhs: $t) {
                    *self = *self - rhs;
                }
            }

            impl<M: Modulus> Mul<$t> for ModInt<M> {
                type Output = Self;

                fn mul(self, rhs: $t) -> Self::Output {
                    let value = self.get() * rhs as u64 % M::VALUE.get();
                    Self::new(value)
                }
            }

            impl<M: Modulus> Mul<ModInt<M>> for $t {
                type Output = ModInt<M>;

                fn mul(self, rhs: ModInt<M>) -> Self::Output {
                    rhs * self
                }
            }

            impl<M: Modulus> MulAssign<$t> for ModInt<M> {
                fn mul_assign(&mut self, rhs: $t) {
                    *self = *self * rhs;
                }
            }

            impl<M: Modulus> Div<$t> for ModInt<M> {
                type Output = Self;

                #[allow(clippy::suspicious_arithmetic_impl)]
                fn div(self, rhs: $t) -> Self::Output {
                    self * Self::new(rhs).inv()
                }
            }

            impl<M: Modulus> Div<ModInt<M>> for $t {
                type Output = ModInt<M>;

                fn div(self, rhs: ModInt<M>) -> Self::Output {
                    (rhs / self).inv()
                }
            }


            impl<M: Modulus> DivAssign<$t> for ModInt<M> {
                fn div_assign(&mut self, rhs: $t) {
                    *self = *self / rhs
                }
            }

            impl<M: Modulus> Sum<$t> for ModInt<M> {
                fn sum<I>(iter: I) -> Self
                where
                    I: Iterator<Item = $t>,
                {
                    iter.fold(Self::zero(), |acc, x| acc + x)
                }
            }

            impl<M: Modulus> Product<$t> for ModInt<M> {
                fn product<I>(iter: I) -> Self
                where
                    I: Iterator<Item = $t>,
                {
                    iter.fold(Self::one(), |acc, x| acc * x)
                }
            }

            impl<M: Modulus> Pow<$t> for ModInt<M> {
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
            impl<M: Modulus> Add<$t> for ModInt<M> {
                type Output = Self;

                fn add(self, rhs: $t) -> Self::Output {
                    self + Self::new(rhs)
                }
            }

            impl<M: Modulus> AddAssign<$t> for ModInt<M> {
                fn add_assign(&mut self, rhs: $t) {
                    *self = *self + rhs;
                }
            }


            impl<M: Modulus> Sub<$t> for ModInt<M> {
                type Output = Self;

                fn sub(self, rhs: $t) -> Self::Output {
                    self - Self::new(rhs)
                }
            }

            impl<M: Modulus> SubAssign<$t> for ModInt<M> {
                fn sub_assign(&mut self, rhs: $t) {
                    *self = *self - rhs;
                }
            }

            impl<M: Modulus> Mul<$t> for ModInt<M> {
                type Output = Self;

                fn mul(self, rhs: $t) -> Self::Output {
                    self * Self::new(rhs)
                }
            }

            impl<M: Modulus> MulAssign<$t> for ModInt<M> {
                fn mul_assign(&mut self, rhs: $t) {
                    *self = *self * rhs;
                }
            }

            impl<M: Modulus> Div<$t> for ModInt<M> {
                type Output = Self;

                #[allow(clippy::suspicious_arithmetic_impl)]
                fn div(self, rhs: $t) -> Self::Output {
                    self * Self::new(rhs).inv()
                }
            }

            impl<M: Modulus> DivAssign<$t> for ModInt<M> {
                fn div_assign(&mut self, rhs: $t) {
                    *self = *self / rhs
                }
            }

            impl<M: Modulus> Sum<$t> for ModInt<M> {
                fn sum<I>(iter: I) -> Self
                where
                    I: Iterator<Item = $t>,
                {
                    iter.fold(Self::zero(), |acc, x| acc + x)
                }
            }

            impl<M: Modulus> Product<$t> for ModInt<M> {
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

impl<M: Modulus> Neg for ModInt<M> {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self::zero() - self
    }
}

impl<M: Modulus> Zero for ModInt<M> {
    fn zero() -> Self {
        Self::new(0)
    }

    fn is_zero(&self) -> bool {
        self.get() == 0
    }
}

impl<M: Modulus> One for ModInt<M> {
    fn one() -> Self {
        Self::new(1)
    }

    fn is_one(&self) -> bool {
        self.get() == 1
    }
}

impl<M: Modulus> Inv for ModInt<M> {
    type Output = Self;

    fn inv(self) -> Self::Output {
        // dbg!(i64::extended_gcd(&self.get(), &M::VALUE.get()));
        // self.pow(M::VALUE.get() - 2)
        if self.get() == 0 {
            panic!("attempt to divide by zero")
        } else {
            debug_assert!(self.get().gcd(&M::VALUE.get()) == 1);
            Self::new(Integer::extended_gcd(&(self.get() as i64), &(M::VALUE.get() as i64)).x)
        }
    }
}

impl<M> vis::visualize::Visualize for ModInt<M> {
    fn visualize(&self) -> String {
        self.get().visualize()
    }
}

pub mod factorial;
pub mod table;
