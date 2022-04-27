use super::PrimeSieve;

#[derive(Debug, Clone)]
pub struct SieveOfEratosthenes {
    limit: usize,
    is_prime: Box<[bool]>,
    primes: Box<[usize]>,
}

impl SieveOfEratosthenes {
    pub fn limit(&self) -> usize {
        self.limit
    }

    pub fn new(limit: usize) -> Self {
        let mut is_prime = vec![true; limit + 1];
        is_prime[0] = false;
        is_prime[1] = false;

        let cap = if limit > 1 {
            let x = limit as f64;
            (x / x.ln() * (1.0 + 1.2762 / x.ln())) as usize
        } else {
            0
        };
        let mut primes = Vec::with_capacity(cap);

        let mut cur = 2;
        while cur * cur <= limit {
            if is_prime[cur] {
                primes.push(cur);

                (2..)
                    .map(|x| x * cur)
                    .take_while(|&x| x <= limit)
                    .for_each(|x| {
                        is_prime[x] = false;
                    });
            }
            cur += 1;
        }

        for (i, _) in is_prime
            .iter()
            .enumerate()
            .take(limit + 1)
            .skip(cur)
            .filter(|&(_, &e)| e)
        {
            primes.push(i);
        }

        Self {
            limit,
            is_prime: is_prime.into_boxed_slice(),
            primes: primes.into_boxed_slice(),
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
