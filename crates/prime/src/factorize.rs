pub struct Factorized<T> {
    factors: Vec<(T, usize)>,
}

impl<T> Factorized<T> {
    pub fn factors(&self) -> &[(T, usize)] {
        &self.factors
    }
}

impl<T> IntoIterator for Factorized<T> {
    type Item = (T, usize);

    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.factors.into_iter()
    }
}

pub enum FactorizationError {
    Zero,
    One,
}

pub trait Factorization: Sized {
    fn factorize(&self) -> Result<Factorized<Self>, FactorizationError>;
}

macro_rules! impl_factorize_for_uint {
($($t:ty),*) => {
    $(
        impl Factorization for $t {
            fn factorize(&self) -> Result<Factorized<Self>, FactorizationError> {
                if self == &0 {
                    Err(FactorizationError::Zero)
                } else if self == &1 {
                    Err(FactorizationError::One)
                } else {
                    let mut factors = vec![];
                    let mut x = self.clone();
                    let mut d = 2;
                    while x > 1 {
                        let mut c = 0usize;
                        while x % d == 0 {
                            x /= d;
                            c += 1;
                        }
                        if c > 0 {
                            factors.push((d, c))
                        }
                        d += 1;
                    }

                    Ok(Factorized { factors })
                }
            }
        }
    )*
};
}

impl_factorize_for_uint!(usize, u8, u16, u32, u64, u128);
