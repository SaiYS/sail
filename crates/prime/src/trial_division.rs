pub trait TrialDivision {
    fn is_prime(self) -> bool;
}

macro_rules! impl_trial_division_for_uint {
    ($($t:ty),*) => {
        $(
            impl TrialDivision for $t {
                fn is_prime(self) -> bool {
                    if self == 0 || self == 1 {
                        false
                    } else if self == 2 || self == 3 {
                        true
                    } else if self & 1 == 0 || self % 3 == 0 {
                        false
                    } else {
                        (6..)
                            .step_by(6)
                            .map(|x| vec![x - 1, x + 1])
                            .flatten()
                            .take_while(|&x| x * x <= self)
                            .all(|x| self % x != 0)
                    }
                }
            }
        )*
    };
}

impl_trial_division_for_uint!(usize, u8, u16, u32, u64, u128);
