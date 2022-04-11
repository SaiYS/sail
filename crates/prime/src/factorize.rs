#[derive(Debug, Clone)]
pub struct Factors<T>(pub Vec<(T, usize)>);

impl<T> Factors<T> {
    pub fn new(factors: Vec<(T, usize)>) -> Self {
        Self(factors)
    }
    pub fn factors(self) -> Vec<(T, usize)> {
        self.0
    }
}

impl<T> IntoIterator for Factors<T> {
    type Item = (T, usize);

    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.factors().into_iter()
    }
}

#[derive(Debug, Clone, Copy)]
pub enum FactorizationError {
    Zero,
    One,
}

pub trait Factorization: Sized {
    fn factorize(&self) -> Result<Factors<Self>, FactorizationError>;
}

macro_rules! impl_factorize_for_uint {
($($t:ty),*) => {
    $(
        impl Factorization for $t {
            fn factorize(&self) -> Result<Factors<Self>, FactorizationError> {
                if self == &0 {
                    Err(FactorizationError::Zero)
                } else if self == &1 {
                    Err(FactorizationError::One)
                } else {
                    let mut factors = vec![];
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
                            factors.push((d, c))
                        }
                        d += 1;
                    }

                    if x > 1 {
                        factors.push((x, 1));
                    }

                    Ok(Factors(factors))
                }
            }
        }
    )*
};
}

impl_factorize_for_uint!(usize, u8, u16, u32, u64, u128);

#[test]
fn factorize() {
    let n = 17u64;
    let factors = n.factorize().ok().unwrap().factors();
    assert_eq!(factors, vec![(17, 1)]);
}
