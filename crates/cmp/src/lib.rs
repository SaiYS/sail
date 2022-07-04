pub trait MaxAssign {
    fn max_assign(&mut self, other: Self);
}

impl<T: Sized + PartialOrd> MaxAssign for T {
    fn max_assign(&mut self, other: Self) {
        if *self < other {
            *self = other;
        }
    }
}

pub trait MinAssign {
    fn min_assign(&mut self, other: Self);
}

impl<T: Sized + PartialOrd> MinAssign for T {
    fn min_assign(&mut self, other: Self) {
        if *self > other {
            *self = other;
        }
    }
}

/// max function for variable arguments.
///
/// ```
/// assert_eq!(4, cmp::max!(3, 1, 4));
/// ```
#[macro_export]
macro_rules! max {
    ($($val:expr),+) => {
        [$($val),+].iter().max().unwrap().clone()
    };
}

/// min function for variable arguments.
///
/// ```
/// assert_eq!(1, cmp::min!(3, 1, 4));
/// ```
#[macro_export]
macro_rules! min {
    ($($val:expr),+) => {
        [$($val),+].iter().min().unwrap().clone()
    };
}
