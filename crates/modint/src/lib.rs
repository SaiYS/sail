use num_integer::Integer;
use num_traits::{Inv, One, Pow, Zero};
use std::{
    convert::TryInto,
    fmt::{Debug, Display, Formatter},
    iter::{Product, Sum},
    num::ParseIntError,
    ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign},
    str::FromStr,
};

pub mod factorial;
pub mod table;

pub const MOD1000000007: u64 = 1000000007;
pub type ModInt1000000007 = StaticModInt<MOD1000000007>;

pub const MOD998244353: u64 = 998244353;
pub type ModInt998244353 = StaticModInt<MOD998244353>;

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct StaticModInt<const M: u64>(u64);

impl<const M: u64> StaticModInt<M> {
    pub const MOD: u64 = M;

    pub fn get(&self) -> u64 {
        self.0
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

        let value = if value > M as i64 {
            value as u64 % M
        } else if value >= 0 {
            value as u64
        } else {
            M - (value.abs() as u64 % M)
        };

        Self(value)
    }
}

macro_rules! impl_from_primitive {
    ($($t:ty),*) => {
        $(
            impl<const M: u64> From<$t> for StaticModInt<M> {
                fn from(value: $t) -> Self {
                    Self::new(value)
                }
            }
        )*
    };
}

impl_from_primitive!(u8, u16, u32, u64, usize, i8, i16, i32, i64, isize);

impl<const M: u64> FromStr for StaticModInt<M> {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let value: i64 = s.parse()?;
        Ok(Self::new(value))
    }
}

impl<const M: u64> Debug for StaticModInt<M> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} (mod. {})", self.get(), M)
    }
}

impl<const M: u64> Display for StaticModInt<M> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        Display::fmt(&self.get(), f)
    }
}

impl<const M: u64> Add for StaticModInt<M> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self::new(if self.get() + rhs.get() < M {
            self.get() + rhs.get()
        } else {
            self.get() + rhs.get() - M
        })
    }
}

impl<const M: u64> AddAssign for StaticModInt<M> {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}

impl<const M: u64> Sub for StaticModInt<M> {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self::new(if self.get() >= rhs.get() {
            self.get() - rhs.get()
        } else {
            self.get() + M - rhs.get()
        })
    }
}

impl<const M: u64> SubAssign for StaticModInt<M> {
    fn sub_assign(&mut self, rhs: Self) {
        *self = *self - rhs;
    }
}

impl<const M: u64> Mul for StaticModInt<M> {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        let value = (self.get() * rhs.get()) % M;
        Self::new(value)
    }
}

impl<const M: u64> MulAssign for StaticModInt<M> {
    fn mul_assign(&mut self, rhs: Self) {
        *self = *self * rhs;
    }
}

impl<const M: u64> Div for StaticModInt<M> {
    type Output = Self;

    #[allow(clippy::suspicious_arithmetic_impl)]
    fn div(self, rhs: Self) -> Self::Output {
        self * rhs.inv()
    }
}

impl<const M: u64> DivAssign for StaticModInt<M> {
    fn div_assign(&mut self, rhs: Self) {
        *self = *self / rhs
    }
}

impl<const M: u64> Sum for StaticModInt<M> {
    fn sum<I>(iter: I) -> Self
    where
        I: Iterator<Item = Self>,
    {
        iter.fold(Self::zero(), |acc, x| acc + x)
    }
}

impl<const M: u64> Product for StaticModInt<M> {
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
            impl<const M: u64> Add<$t> for StaticModInt<M> {
                type Output = Self;

                fn add(self, rhs: $t) -> Self::Output {
                    let value = self.get() + rhs as u64;
                    Self::new(if value < M {
                        value
                    } else {
                        value - M
                    })
                }
            }

            impl<const M: u64> Add<StaticModInt<M>> for $t {
                type Output = StaticModInt<M>;

                fn add(self, rhs: StaticModInt<M>) -> Self::Output {
                    rhs + self
                }
            }

            impl<const M: u64> AddAssign<$t> for StaticModInt<M> {
                fn add_assign(&mut self, rhs: $t) {
                    *self = *self + rhs;
                }
            }

            impl<const M: u64> Sub<$t> for StaticModInt<M> {
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

            impl<const M: u64> Sub<StaticModInt<M>> for $t {
                type Output = StaticModInt<M>;

                fn sub(self, rhs: StaticModInt<M>) -> Self::Output {
                    -(rhs - self)
                }
            }

            impl<const M: u64> SubAssign<$t> for StaticModInt<M> {
                fn sub_assign(&mut self, rhs: $t) {
                    *self = *self - rhs;
                }
            }

            impl<const M: u64> Mul<$t> for StaticModInt<M> {
                type Output = Self;

                fn mul(self, rhs: $t) -> Self::Output {
                    let value = self.get() * rhs as u64 % M;
                    Self::new(value)
                }
            }

            impl<const M: u64> Mul<StaticModInt<M>> for $t {
                type Output = StaticModInt<M>;

                fn mul(self, rhs: StaticModInt<M>) -> Self::Output {
                    rhs * self
                }
            }

            impl<const M: u64> MulAssign<$t> for StaticModInt<M> {
                fn mul_assign(&mut self, rhs: $t) {
                    *self = *self * rhs;
                }
            }

            impl<const M: u64> Div<$t> for StaticModInt<M> {
                type Output = Self;

                #[allow(clippy::suspicious_arithmetic_impl)]
                fn div(self, rhs: $t) -> Self::Output {
                    self * Self::new(rhs).inv()
                }
            }

            impl<const M: u64> Div<StaticModInt<M>> for $t {
                type Output = StaticModInt<M>;

                fn div(self, rhs: StaticModInt<M>) -> Self::Output {
                    (rhs / self).inv()
                }
            }

            impl<const M: u64> DivAssign<$t> for StaticModInt<M> {
                fn div_assign(&mut self, rhs: $t) {
                    *self = *self / rhs
                }
            }

            impl<const M: u64> Sum<$t> for StaticModInt<M> {
                fn sum<I>(iter: I) -> Self
                where
                    I: Iterator<Item = $t>,
                {
                    iter.fold(Self::zero(), |acc, x| acc + x)
                }
            }

            impl<const M: u64> Product<$t> for StaticModInt<M> {
                fn product<I>(iter: I) -> Self
                where
                    I: Iterator<Item = $t>,
                {
                    iter.fold(Self::one(), |acc, x| acc * x)
                }
            }

            impl<const M: u64> Pow<$t> for StaticModInt<M> {
                type Output = Self;

                fn pow(self, mut exp: $t) -> Self::Output {
                    let mut res = StaticModInt::one();
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
            impl<const M: u64> Add<$t> for StaticModInt<M> {
                type Output = Self;

                fn add(self, rhs: $t) -> Self::Output {
                    self + Self::new(rhs)
                }
            }

            impl<const M: u64> AddAssign<$t> for StaticModInt<M> {
                fn add_assign(&mut self, rhs: $t) {
                    *self = *self + rhs;
                }
            }

            impl<const M: u64> Sub<$t> for StaticModInt<M> {
                type Output = Self;

                fn sub(self, rhs: $t) -> Self::Output {
                    self - Self::new(rhs)
                }
            }

            impl<const M: u64> SubAssign<$t> for StaticModInt<M> {
                fn sub_assign(&mut self, rhs: $t) {
                    *self = *self - rhs;
                }
            }

            impl<const M: u64> Mul<$t> for StaticModInt<M> {
                type Output = Self;

                fn mul(self, rhs: $t) -> Self::Output {
                    self * Self::new(rhs)
                }
            }

            impl<const M: u64> MulAssign<$t> for StaticModInt<M> {
                fn mul_assign(&mut self, rhs: $t) {
                    *self = *self * rhs;
                }
            }

            impl<const M: u64> Div<$t> for StaticModInt<M> {
                type Output = Self;

                #[allow(clippy::suspicious_arithmetic_impl)]
                fn div(self, rhs: $t) -> Self::Output {
                    self * Self::new(rhs).inv()
                }
            }

            impl<const M: u64> DivAssign<$t> for StaticModInt<M> {
                fn div_assign(&mut self, rhs: $t) {
                    *self = *self / rhs
                }
            }

            impl<const M: u64> Sum<$t> for StaticModInt<M> {
                fn sum<I>(iter: I) -> Self
                where
                    I: Iterator<Item = $t>,
                {
                    iter.fold(Self::zero(), |acc, x| acc + x)
                }
            }

            impl<const M: u64> Product<$t> for StaticModInt<M> {
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

impl<const M: u64> Neg for StaticModInt<M> {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self::zero() - self
    }
}

impl<const M: u64> Zero for StaticModInt<M> {
    fn zero() -> Self {
        Self::new(0)
    }

    fn is_zero(&self) -> bool {
        self.get() == 0
    }
}

impl<const M: u64> One for StaticModInt<M> {
    fn one() -> Self {
        Self::new(1)
    }

    fn is_one(&self) -> bool {
        self.get() == 1
    }
}

impl<const M: u64> Inv for StaticModInt<M> {
    type Output = Self;

    fn inv(self) -> Self::Output {
        // dbg!(i64::extended_gcd(&self.get(), &M::VALUE.get()));
        // self.pow(M::VALUE.get() - 2)
        if self.get() == 0 {
            panic!("attempt to divide by zero")
        } else {
            debug_assert!(self.get().gcd(&M) == 1);
            Self::new(Integer::extended_gcd(&(self.get() as i64), &(M as i64)).x)
        }
    }
}
