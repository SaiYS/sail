use crate::{
    property::{Associativity, Cancellativity, Commutativity, Identity, Invertibility},
    Operation,
};

pub trait Magma: Operation + From<Self::T> {
    type T: Clone;
    fn get(self) -> Self::T;
}

pub trait SemiGroup: Associativity + From<Self::T> {
    type T: Clone;
    fn get(self) -> Self::T;
}

impl<S: SemiGroup> Magma for S {
    type T = <S as SemiGroup>::T;

    fn get(self) -> Self::T {
        SemiGroup::get(self)
    }
}

pub trait Monoid: Associativity + Identity + From<Self::T> {
    type T: Clone;
    fn get(self) -> Self::T;
    fn identity() -> Self {
        Identity::identity()
    }
    fn fold_right(xs: &[Self]) -> Self {
        if let Some((x, rest)) = xs.split_last() {
            x.clone().operate(Self::fold_right(rest))
        } else {
            Monoid::identity()
        }
    }
    fn fold_left(xs: &[Self]) -> Self {
        if let Some((x, rest)) = xs.split_first() {
            x.clone().operate(Self::fold_left(rest))
        } else {
            Monoid::identity()
        }
    }
}

impl<M: Monoid> SemiGroup for M {
    type T = <M as Monoid>::T;

    fn get(self) -> Self::T {
        Monoid::get(self)
    }
}

pub trait Group:
    Associativity + Identity + Invertibility + Cancellativity + From<Self::T>
{
    type T: Clone;
    fn get(self) -> Self::T;
    fn identity() -> Self {
        Identity::identity()
    }
    fn fold_right(xs: &[Self]) -> Self {
        if let Some((x, rest)) = xs.split_last() {
            x.clone().operate(Self::fold_right(rest))
        } else {
            Group::identity()
        }
    }
    fn fold_left(xs: &[Self]) -> Self {
        if let Some((x, rest)) = xs.split_first() {
            x.clone().operate(Self::fold_left(rest))
        } else {
            Group::identity()
        }
    }
}

impl<G: Group> Monoid for G {
    type T = <G as Group>::T;

    fn get(self) -> Self::T {
        Group::get(self)
    }
}

pub trait AbelianGroup:
    Associativity + Identity + Invertibility + Commutativity + Cancellativity + From<Self::T>
{
    type T: Clone;
    fn get(self) -> Self::T;
    fn identity() -> Self {
        Identity::identity()
    }
    fn fold(xs: &[Self]) -> Self {
        if let Some((x, rest)) = xs.split_first() {
            x.clone().operate(Self::fold(rest))
        } else {
            AbelianGroup::identity()
        }
    }
}

impl<A: AbelianGroup> Group for A {
    type T = <A as AbelianGroup>::T;

    fn get(self) -> Self::T {
        AbelianGroup::get(self)
    }
}

pub trait QuasiGroup: Cancellativity + From<Self::T> {
    type T: Clone;
    fn get(self) -> Self::T;
}

pub trait Loop: Identity + Invertibility + Cancellativity + From<Self::T> {
    type T: Clone;
    fn get(self) -> Self::T;
}

impl<L: Loop> QuasiGroup for L {
    type T = <L as Loop>::T;

    fn get(self) -> Self::T {
        Loop::get(self)
    }
}
