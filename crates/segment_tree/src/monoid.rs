use num_traits::Bounded;

pub trait Monoid {
    type T;
    fn identity() -> Self;
    fn binary_operation(x: Self, y: Self) -> Self;
    fn get(self) -> Self::T;
    fn set(value: Self::T) -> Self;
    // associativity: x + (y + z) == (x + y) + z
}

#[derive(Debug, Clone)]
pub struct Min<T>(T);

impl<T: Ord + Bounded> Monoid for Min<T> {
    type T = T;

    fn identity() -> Self {
        Self(T::max_value())
    }

    fn binary_operation(x: Self, y: Self) -> Self {
        Self(x.get().min(y.get()))
    }

    fn get(self) -> Self::T {
        self.0
    }

    fn set(value: Self::T) -> Self {
        Self(value)
    }
}
