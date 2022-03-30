pub trait PrimeSieve {
    fn limit(&self) -> usize;
    fn is_prime(&self, n: usize) -> bool;
    fn primes(&self) -> &[usize];
}

pub mod atkin;
pub mod eratosthenes;