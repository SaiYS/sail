use super::group::{GProduct, GSum, Group};

pub trait AbelianGroup: Group {}

pub type ASum<T> = GSum<T>;
pub type AProduct<T> = GProduct<T>;
