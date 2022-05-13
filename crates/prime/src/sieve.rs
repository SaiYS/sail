use super::factorize::{FactorizationError, PrimeFactors};
use std::collections::BTreeMap;

pub mod atkin;
pub mod eratosthenes;

pub trait PrimeSieve {
    fn limit(&self) -> usize;
    fn is_prime(&self, n: usize) -> bool;
    fn primes(&self) -> &[usize];

    fn factorize(&self, mut n: usize) -> Result<PrimeFactors<usize>, FactorizationError> {
        assert!(n <= self.limit() * self.limit());
        if n == 0 {
            Err(FactorizationError::Zero)
        } else if n == 1 {
            Err(FactorizationError::One)
        } else {
            let mut factors = BTreeMap::new();
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
                    factors.insert(p, c);
                }
            }

            if n != 1 {
                factors.insert(n, 1);
            }

            Ok(PrimeFactors(factors))
        }
    }
}
