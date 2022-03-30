use crate::factorize::{FactorizationError, Factorized};

pub trait PrimeSieve {
    fn limit(&self) -> usize;
    fn is_prime(&self, n: usize) -> bool;
    fn primes(&self) -> &[usize];

    fn factorize(&self, mut n: usize) -> Result<Factorized<usize>, FactorizationError> {
        assert!(n <= self.limit() * self.limit());
        if n == 0 {
            Err(FactorizationError::Zero)
        } else if n == 1 {
            Err(FactorizationError::One)
        } else {
            let mut factors = Vec::new();
            for &p in self.primes().iter() {
                if p * p > n {
                    break;
                }
                let mut c = 0usize;
                while n % p == 0 {
                    n /= p;
                    c += 1;
                }

                if c > 0 {
                    factors.push((p, c));
                }
            }

            if n != 1 {
                factors.push((n, 1));
            }

            Ok(Factorized { factors })
        }
    }
}

pub mod atkin;
pub mod eratosthenes;
