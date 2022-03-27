#[derive(Debug, Clone)]
pub struct SieveOfEratosthenes {
    len: usize,
    is_prime: Vec<bool>,
    primes: Vec<usize>,
}

impl SieveOfEratosthenes {
    pub fn len(&self) -> usize {
        self.len
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

                (2..).map(|x| x * cur).take_while(|&x| x <= n).for_each(|x| {
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
            len: n,
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

#[test]
fn dbg() {
    let s = SieveOfEratosthenes::new(100000);
    dbg!(&s);
}
