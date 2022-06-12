use std::collections::btree_map::{BTreeMap, IntoIter};

#[derive(Debug, Clone)]
pub struct PrimeFactors<T>(pub BTreeMap<T, usize>);

impl<T> PrimeFactors<T> {
    pub fn primt_factors(self) -> BTreeMap<T, usize> {
        self.0
    }
}

impl<T> IntoIterator for PrimeFactors<T> {
    type Item = (T, usize);

    type IntoIter = IntoIter<T, usize>;

    fn into_iter(self) -> Self::IntoIter {
        self.primt_factors().into_iter()
    }
}

#[derive(Debug, Clone, Copy)]
pub enum FactorizationError {
    Zero,
    One,
}

pub trait Factorization: Sized {
    fn factorize(&self) -> Result<PrimeFactors<Self>, FactorizationError>;
}

macro_rules! impl_factorize_for_uint {
    ($($t:ty),*) => {
        $(
            impl Factorization for $t {
                fn factorize(&self) -> Result<PrimeFactors<Self>, FactorizationError> {
                    if self == &0 {
                        Err(FactorizationError::Zero)
                    } else if self == &1 {
                        Err(FactorizationError::One)
                    } else {
                        let mut primt_factors = BTreeMap::new();
                        let mut x = self.clone();
                        let mut d = 2;
                        loop {
                            if d * d > x {
                                break;
                            }
                            let mut c = 0usize;
                            while x % d == 0 {
                                x /= d;
                                c += 1;
                            }
                            if c > 0 {
                                primt_factors.insert(d, c);
                            }
                            d += 1;
                        }

                        if x > 1 {
                            primt_factors.insert(x, 1);
                        }

                        Ok(PrimeFactors(primt_factors))
                    }
                }
            }
        )*
    };
}

impl_factorize_for_uint!(usize, u8, u16, u32, u64, u128);
