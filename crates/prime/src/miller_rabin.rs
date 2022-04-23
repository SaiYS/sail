use rand::Rng;

pub trait MillerRabin {
    fn is_prime<R: Rng>(&self, accuracy: usize, rng: &mut R) -> bool;
}

macro_rules! impl_miller_rabin_for_uint {
    ($($t:ty),*) => {
        $(
            impl MillerRabin for $t {
                fn is_prime<R: rand::Rng>(&self, k: usize, rng: &mut R) -> bool {
                    let n = *self;
                    if n == 2 {
                        true
                    } else if n == 0 || n == 1 || n & 1 == 0 {
                        false
                    } else {
                        let s = (n - 1).trailing_zeros();
                        let d = (n - 1) >> s;

                        for _ in 0..k {
                            let a = rng.gen_range(1, n);
                            let mut y = mod_pow(a as u128, d as u128, n as u128) as $t;
                            if y != 1 && y != n - 1 && {
                                (0..s).all(|_| {
                                    y *= y;
                                    y %= n;
                                    y != n - 1
                                })
                            } {
                                return false;
                            }
                        }

                        true
                    }
                }
            }
        )*
    };
}

impl_miller_rabin_for_uint!(usize, u32, u64, u128);

fn mod_pow(mut x: u128, mut y: u128, m: u128) -> u128 {
    let mut res = 1;
    while y != 0 {
        if y & 1 != 0 {
            res *= x;
            res %= m;
        }
        x *= x;
        x %= m;
        y >>= 1;
    }

    res
}
