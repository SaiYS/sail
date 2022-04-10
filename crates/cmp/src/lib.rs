pub trait MaxAssign {
    fn max_assign(&mut self, other: Self);
}

impl<T: Sized + Clone + Ord> MaxAssign for T {
    fn max_assign(&mut self, other: Self) {
        *self = std::cmp::max(self.clone(), other);
    }
}

pub trait MaxByKeyAssign {
    fn max_assign<F: FnMut(&Self) -> K, K: Ord>(&mut self, other: Self, f: F);
}

impl<T: Sized + Clone + Ord> MaxByKeyAssign for T {
    fn max_assign<F: FnMut(&T) -> K, K: Ord>(&mut self, other: Self, f: F) {
        *self = std::cmp::max_by_key(self.clone(), other, f);
    }
}

pub trait MinAssign {
    fn min_assign(&mut self, other: Self);
}

impl<T: Sized + Clone + Ord> MinAssign for T {
    fn min_assign(&mut self, other: Self) {
        *self = std::cmp::min(self.clone(), other);
    }
}

pub trait MinByKeyAssign {
    fn min_assign<F: FnMut(&Self) -> K, K: Ord>(&mut self, other: Self, f: F);
}

impl<T: Sized + Clone + Ord> MinByKeyAssign for T {
    fn min_assign<F: FnMut(&T) -> K, K: Ord>(&mut self, other: Self, f: F) {
        *self = std::cmp::min_by_key(self.clone(), other, f);
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
