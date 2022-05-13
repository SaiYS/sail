/// Find primes and factors in trial division
///
/// Complexity `O(sqrt(n))`
pub trait TrialDivision: Sized {
    fn is_prime(&self) -> bool;

    fn factors(&self) -> Vec<Self>;
}

macro_rules! impl_trial_division_for_uint {
    ($($t:ty),*) => {
        $(
            impl TrialDivision for $t {
                fn is_prime(&self) -> bool {
                    if *self == 0 || *self == 1 {
                        false
                    } else if *self == 2 || *self == 3 {
                        true
                    } else if *self & 1 == 0 || *self % 3 == 0 {
                        false
                    } else {
                        (6..)
                            .step_by(6)
                            .map(|x| vec![x - 1, x + 1])
                            .flatten()
                            .take_while(|&x| x * x <= *self)
                            .all(|x| *self % x != 0)
                    }
                }


                fn factors(&self) -> Vec<Self> {
                    let mut a = vec![];
                    let mut b = vec![];
                    for d in (1..).take_while(|&x| x * x <= *self) {
                        if self % d == 0 {
                            a.push(d);
                            if *self != d * d {
                                b.push(self / d);
                            }
                        }
                    }

                    b.reverse();
                    a.append(&mut b);
                    a
                }
            }
        )*
    };
}

impl_trial_division_for_uint!(usize, u8, u16, u32, u64, u128);

#[test]
fn test_factors() {
    let a = 24usize;
    assert_eq!(a.factors(), vec![1, 2, 3, 4, 6, 8, 12, 24]);
    let a = 9usize;
    assert_eq!(a.factors(), vec![1, 3, 9]);
    let a = 17usize;
    assert_eq!(a.factors(), vec![1, 17]);
}
