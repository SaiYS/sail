use rand::Rng;
use std::ops::{Range, RangeInclusive};

pub mod marker;

pub trait Generable {
    type Output;
    fn gen<R: Rng>(self, rng: &mut R) -> Self::Output;
}

macro_rules! impl_generable_for_prim_int {
    ($($t:ty),*) => {
        $(
            impl Generable for $t {
                type Output = $t;
                fn gen<R: Rng>(self, _rng: &mut R) -> Self::Output {
                    self
                }
            }
        )*
    };
}

impl_generable_for_prim_int!(usize, u8, u16, u32, u64, u128, isize, i8, i16, i32, i64, i128);

impl<T: Clone> Generable for &[T] {
    type Output = Vec<T>;

    fn gen<R: Rng>(self, _rng: &mut R) -> Self::Output {
        self.to_vec()
    }
}

macro_rules! impl_generable_for_range_expr {
    ($($t:ty),*) => {
        $(
            impl Generable for Range<$t> {
                type Output = $t;
                fn gen<R: Rng>(self, rng: &mut R) -> Self::Output {
                    rng.gen_range(self.start, self.end)
                }
            }

            impl Generable for RangeInclusive<$t> {
                type Output = $t;
                fn gen<R: Rng>(self, rng: &mut R) -> Self::Output {
                    rng.gen_range(self.start(), self.end() + 1)
                }
            }
        )*
    };
}

impl_generable_for_range_expr!(usize, u8, u16, u32, u64, u128, isize, i8, i16, i32, i64, i128);

#[macro_export]
macro_rules! gen {
    () => {};
    (@rng [$rng:expr]) => {};

    // repeat pattern
    // foo = [bar; n]
    (@rng [$rng:expr] $name:tt = [$generable:expr; $rep:expr], $($rest:tt)*) => {
        let $name = (0..$rep as usize).map(|_| $generable.gen($rng)).collect::<Vec<_>>();
        gen! {
            @rng [$rng]
            $($rest)*
        }
    };

    (@rng [$rng:expr] $name:tt = [$generable:expr; $rep:expr]) => {
        let $name = (0..$rep as usize).map(|_| $generable.gen($rng)).collect::<Vec<_>>();
    };

    (@rng [$rng:expr] mut $name:tt = [$generable:expr; $rep:expr], $($rest:tt)*) => {
        let mut $name = (0..$rep as usize).map(|_| $generable.gen($rng)).collect::<Vec<_>>();
        gen! {
            @rng [$rng]
            $($rest)*
        }
    };

    (@rng [$rng:expr] mut $name:tt = [$generable:expr; $rep:expr]) => {
        let mut $name = (0..$rep as usize).map(|_| $generable.gen($rng)).collect::<Vec<_>>();
    };

    // fixed pattern
    //
    (@rng [$rng:expr] $name:tt = $generable:expr, $($rest:tt)*) => {
        let $name = $generable.gen($rng);
        gen! {
            @rng [$rng]
            $($rest)*
        }
    };

    (@rng [$rng:expr] $name:tt = $generable:expr) => {
        let $name = $generable.gen($rng);
    };

    (@rng [$rng:expr] mut $name:tt = $generable:expr, $($rest:tt)*) => {
        let mut $name = $generable.gen($rng);
        gen! {
            @rng [$rng]
            $($rest)*
        }
    };

    (@rng [$rng:expr] mut $name:tt = $generable:expr) => {
        let mut $name = $generable.gen($rng);
    };

    // initialize rng
    ($($rest:tt)+) => {
        let mut rng = rand::thread_rng();
        gen! {
            @rng [&mut rng]
            $($rest)+
        }
        drop(rng);
    };
}

#[test]
fn example() {
    gen! {
        a = [0usize..=10; 10],
        b = 1usize..=1000,
        c = [1, 2, 3],
    }

    dbg!(a, b, c);
}
