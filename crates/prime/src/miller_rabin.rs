use rand::Rng;

fn miller_rabin<R: Rng>(n: u128, k: usize, rng: &mut R) -> bool {
    if n == 2 {
        true
    } else if n == 0 || n == 1 || n & 1 == 0 {
        false
    } else {
        let s = (n - 1).trailing_zeros();
        let d = (n - 1) >> s;

        for _ in 0..k {
            let a = rng.gen_range(1..n);
            let mut y = mod_pow(a as u128, d as u128, n as u128);
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

/// # Miller–Rabin algorythm
///
/// Seehttps://en.wikipedia.org/wiki/Miller%e2%80%93Rabin_primality_test
///
/// Estimate whether the number is prime or composite.
/// This algorythm is **non-deterministic**,
/// an argument `accuracy` is used to indicate the accuracy of the judgement.
/// Possibility of incorrect judge with `accuracy = k` is 4 ^ -k at most.
///
/// Complexity: `O(k × log3 n)`
pub trait MillerRabin {
    /// Returns whether `self` is a prime number
    fn is_prime<R: Rng>(&self, accuracy: usize, rng: &mut R) -> bool;
}

macro_rules! impl_miller_rabin_for_uint {
    ($($t:ty),*) => {
        $(
            impl MillerRabin for $t {
                fn is_prime<R: rand::Rng>(&self, k: usize, rng: &mut R) -> bool {
                    miller_rabin(*self as u128, k, rng)
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
