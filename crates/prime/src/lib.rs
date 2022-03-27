pub mod factorize;
pub mod miller_rabin;
pub mod sieve_of_atkin;
pub mod sieve_of_eratosthenes;
pub mod trial_division;

#[cfg(test)]
mod tests {
    use crate::{
        miller_rabin::MillerRabin, sieve_of_atkin::SieveOfAtkin,
        sieve_of_eratosthenes::SieveOfEratosthenes, trial_division::TrialDivision,
    };
    use rand::{thread_rng, Rng};

    #[test]
    fn small_trial_division() {
        assert!(TrialDivision::is_prime(0u64) == false);
        assert!(TrialDivision::is_prime(1u64) == false);
        assert!(TrialDivision::is_prime(2u64) == true);
        assert!(TrialDivision::is_prime(3u64) == true);
        assert!(TrialDivision::is_prime(4u64) == false);
        assert!(TrialDivision::is_prime(5u64) == true);
        assert!(TrialDivision::is_prime(6u64) == false);
        assert!(TrialDivision::is_prime(7u64) == true);
        assert!(TrialDivision::is_prime(8u64) == false);
        assert!(TrialDivision::is_prime(9u64) == false);
        assert!(TrialDivision::is_prime(10u64) == false);
        assert!(TrialDivision::is_prime(11u64) == true);
    }

    #[test]
    fn small_miller_rabin() {
        let accuracy = 20;
        let mut rng = thread_rng();

        assert!(MillerRabin::is_prime(0u64, accuracy, &mut rng) == false);
        assert!(MillerRabin::is_prime(1u64, accuracy, &mut rng) == false);
        assert!(MillerRabin::is_prime(2u64, accuracy, &mut rng) == true);
        assert!(MillerRabin::is_prime(3u64, accuracy, &mut rng) == true);
        assert!(MillerRabin::is_prime(4u64, accuracy, &mut rng) == false);
        assert!(MillerRabin::is_prime(5u64, accuracy, &mut rng) == true);
        assert!(MillerRabin::is_prime(6u64, accuracy, &mut rng) == false);
        assert!(MillerRabin::is_prime(7u64, accuracy, &mut rng) == true);
        assert!(MillerRabin::is_prime(8u64, accuracy, &mut rng) == false);
        assert!(MillerRabin::is_prime(9u64, accuracy, &mut rng) == false);
        assert!(MillerRabin::is_prime(10u64, accuracy, &mut rng) == false);
        assert!(MillerRabin::is_prime(11u64, accuracy, &mut rng) == true);
    }

    #[test]
    fn assert_trial_division_and_miller_rabin() {
        let mut rng = thread_rng();
        for _ in 0..10000 {
            let r: u64 = rng.gen_range(0, 100000000);
            assert_eq!(
                MillerRabin::is_prime(r, 20, &mut rng),
                TrialDivision::is_prime(r)
            );
        }
    }

    #[test]
    fn assert_miller_rabin_and_sieve_of_eratosthenes() {
        let n = 1000000;
        let sieve = SieveOfEratosthenes::new(n);
        let mut rng = thread_rng();

        for i in 0..=n {
            assert_eq!(sieve.is_prime(i), MillerRabin::is_prime(i, 20, &mut rng));
        }
    }

    #[test]
    fn assert_miller_rabin_and_sieve_of_atkin() {
        let n = 1000000;
        let sieve = SieveOfAtkin::new(n);
        let mut rng = thread_rng();

        for i in 0..=n {
            assert_eq!(sieve.is_prime(i), MillerRabin::is_prime(i, 20, &mut rng));
        }
    }

    #[test]
    fn assert_atkin_and_eratosthenes() {
        let n = 1000000;
        let atkin = SieveOfAtkin::new(n);
        let eratosthenes = SieveOfEratosthenes::new(n);

        assert_eq!(atkin.primes(), eratosthenes.primes());
    }
}
