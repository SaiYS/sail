use super::sieve::PrimeSieve;

#[derive(Debug, Clone)]
pub struct SieveOfEratosthenes {
    limit: usize,
    is_prime: Vec<bool>,
    primes: Vec<usize>,
}

impl SieveOfEratosthenes {
    pub fn limit(&self) -> usize {
        self.limit
    }

    pub fn new(n: usize) -> Self {
        let mut is_prime = vec![true; n + 1];
        is_prime[0] = false;
        is_prime[1] = false;
        let mut primes = Vec::new();

        let mut cur = 2;
        while cur * cur <= n {
            if is_prime[cur] {
                primes.push(cur);

                (2..)
                    .map(|x| x * cur)
                    .take_while(|&x| x <= n)
                    .for_each(|x| {
                        is_prime[x] = false;
                    });
            }
            cur += 1;
        }

        for i in cur..=n {
            if is_prime[i] {
                primes.push(i);
            }
        }

        Self {
            limit: n,
            is_prime,
            primes,
        }
    }

    pub fn is_prime(&self, n: usize) -> bool {
        self.is_prime[n]
    }

    pub fn primes(&self) -> &[usize] {
        &self.primes
    }
}

impl PrimeSieve for SieveOfEratosthenes {
    fn limit(&self) -> usize {
        self.limit()
    }

    fn is_prime(&self, n: usize) -> bool {
        self.is_prime(n)
    }

    fn primes(&self) -> &[usize] {
        self.primes()
    }
}
