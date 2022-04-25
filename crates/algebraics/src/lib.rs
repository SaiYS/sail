// S > M > G > A
// pub mod abelian_group;
// pub mod group;
// pub mod monoid;
// pub mod semigroup;

pub trait Operation: Sized + Clone {
    fn operate(self, rhs: Self) -> Self;
    fn operate_assign(&mut self, rhs: Self) {
        *self = self.clone().operate(rhs);
    }
}

pub mod abstruct;
pub mod property;
pub mod structure;
