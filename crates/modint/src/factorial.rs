use super::{ModInt, Modulus};
use num_traits::{One, Zero};

pub trait Factorial: Sized + Clone {
    fn factorial(self) -> Self {
        self.clone().permutation(self)
    }

    fn permutation<T: Into<Self>>(self, k: T) -> Self;

    fn combination<T: Into<Self>>(self, k: T) -> Self;

    fn binomial(self, k: Self) -> Self {
        self.combination(k)
    }
}

macro_rules! impl_factorial {
    ($($t:ty),*) => {
        $(
            impl Factorial for $t {
                fn permutation<T: Into<Self>>(self, k: T) -> Self {
                    let k = k.into();
                    if k > self {
                        0
                    } else {
                        (self - k + 1..=self).product()
                    }
                }

                fn combination<T: Into<Self>>(self, k: T) -> Self {
                    let k = k.into();
                    self.permutation(k) / k.factorial()
                }
            }
        )*
    };
}

impl_factorial!(u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize);

impl<M: Modulus> Factorial for ModInt<M> {
    fn permutation<T: Into<Self>>(self, k: T) -> Self {
        let n = self.get();
        let k = k.into().get();
        if k > n {
            Self::zero()
        } else {
            ((n - k + 1)..=n).fold(Self::one(), |mut acc, x| {
                acc *= x;
                acc
            })
        }
    }

    fn combination<T: Into<Self>>(self, k: T) -> Self {
        let k = k.into();
        self.permutation(k) / k.factorial()
    }
}
