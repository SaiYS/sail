pub fn manhattan_distance(a: (i64, i64), b: (i64, i64)) -> i64 {
    (a.0.abs_diff(b.0) + a.1.abs_diff(b.1)) as i64
}

pub fn euclidean_distance(a: (f64, f64), b: (f64, f64)) -> f64 {
    ((a.0 - b.0).powi(2) + (a.1 - b.1).powi(2)).sqrt()
}

pub trait ManhattanDistance {
    type Output;
    fn manhattan_distance(&self, other: &Self) -> Self::Output;
}

macro_rules! impl_manhattan_distance_for_integer {
    ($($t:ty),*) => {
        $(
            impl ManhattanDistance for ($t, $t) {
                type Output = $t;

                fn manhattan_distance(&self, other: &Self) -> Self::Output {
                    (self.0.abs_diff(other.0) + self.1.abs_diff(other.1)) as _
                }
            }
        )*
    };
}

impl_manhattan_distance_for_integer!(
    usize, u8, u16, u32, u64, u128, isize, i8, i16, i32, i64, i128
);

macro_rules! impl_manhattan_distance_for_float {
    ($($t:ty),*) => {
        $(
            impl ManhattanDistance for ($t, $t) {
                type Output = $t;

                fn manhattan_distance(&self, other: &Self) -> Self::Output {
                    (self.0 - other.0).abs() + (self.1 - other.1).abs()
                }
            }

        )*
    };
}

impl_manhattan_distance_for_float!(f32, f64);

pub trait EuclideanDistance {
    type Output;
    fn euclidean_distance(&self, other: &Self) -> Self::Output;
}

macro_rules! impl_euclidean_distance_for_integer {
    ($($t:ty),*) => {
        $(
            impl EuclideanDistance for ($t, $t) {
                type Output = f64;

                fn euclidean_distance(&self, other: &Self) -> Self::Output {
                    ((self.0.abs_diff(other.0).pow(2) + self.1.abs_diff(other.1).pow(2)) as f64).sqrt()
                }
            }
        )*
    };
}

impl_euclidean_distance_for_integer!(
    usize, u8, u16, u32, u64, u128, isize, i8, i16, i32, i64, i128
);

macro_rules! impl_euclidean_distance_for_float {
    ($($t:ty),*) => {
        $(
            impl EuclideanDistance for ($t, $t) {
                type Output = $t;

                fn euclidean_distance(&self, other: &Self) -> Self::Output {
                    ((self.0 - other.0).powi(2) + (self.1 - other.1).powi(2)).sqrt()
                }
            }

        )*
    };
}

impl_euclidean_distance_for_float!(f32, f64);
