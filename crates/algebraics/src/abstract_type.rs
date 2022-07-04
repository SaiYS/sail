use crate::property::{
    Associativity, Cancellativity, Commutativity, Identity, Invertibility, Operation,
};

pub trait Magma: Operation<Self::I> {
    type I: Clone + PartialEq;
    fn get(self) -> Self::I;
}

pub trait SemiGroup: Associativity<Self::I> {
    type I: Clone + PartialEq;
    fn get(self) -> Self::I;
}

impl<S: SemiGroup> Magma for S {
    type I = <S as SemiGroup>::I;

    fn get(self) -> Self::I {
        SemiGroup::get(self)
    }
}

pub trait Monoid: Associativity<Self::I> + Identity<Self::I> {
    type I: Clone + PartialEq;
    fn get(self) -> Self::I;
    fn identity() -> Self::I {
        <Self as Identity<Self::I>>::identity()
    }
    fn fold_right(xs: &[Self::I]) -> Self::I {
        if let Some((x, rest)) = xs.split_last() {
            Self::operate(x.clone(), Self::fold_right(rest))
        } else {
            <Self as Monoid>::identity()
        }
    }
    fn fold_left(xs: &[Self::I]) -> Self::I {
        if let Some((x, rest)) = xs.split_first() {
            Self::operate(x.clone(), Self::fold_left(rest))
        } else {
            <Self as Monoid>::identity()
        }
    }
}

impl<M: Monoid> SemiGroup for M {
    type I = <M as Monoid>::I;

    fn get(self) -> Self::I {
        Monoid::get(self)
    }
}

pub trait Group:
    Associativity<Self::I> + Identity<Self::I> + Invertibility<Self::I> + Cancellativity<Self::I>
{
    type I: Clone + PartialEq;
    fn get(self) -> Self::I;
    fn identity() -> Self::I {
        <Self as Identity<Self::I>>::identity()
    }
    fn fold_right(xs: &[Self::I]) -> Self::I {
        if let Some((x, rest)) = xs.split_last() {
            Self::operate(x.clone(), Self::fold_right(rest))
        } else {
            <Self as Group>::identity()
        }
    }
    fn fold_left(xs: &[Self::I]) -> Self::I {
        if let Some((x, rest)) = xs.split_first() {
            Self::operate(x.clone(), Self::fold_left(rest))
        } else {
            <Self as Group>::identity()
        }
    }
}

impl<G: Group> Monoid for G {
    type I = <G as Group>::I;

    fn get(self) -> Self::I {
        Group::get(self)
    }
}

pub trait AbelianGroup:
    Associativity<Self::I>
    + Identity<Self::I>
    + Invertibility<Self::I>
    + Commutativity<Self::I>
    + Cancellativity<Self::I>
{
    type I: Clone + PartialEq;
    fn get(self) -> Self::I;
    fn identity() -> Self::I {
        <Self as Identity<Self::I>>::identity()
    }
    fn fold(xs: &[Self::I]) -> Self::I {
        if let Some((x, rest)) = xs.split_first() {
            Self::operate(x.clone(), Self::fold(rest))
        } else {
            <Self as AbelianGroup>::identity()
        }
    }
}

impl<A: AbelianGroup> Group for A {
    type I = <A as AbelianGroup>::I;

    fn get(self) -> Self::I {
        AbelianGroup::get(self)
    }
}

pub trait QuasiGroup: Cancellativity<Self::I> + From<Self::I> {
    type I: Clone + PartialEq;
    fn get(self) -> Self::I;
}

pub trait Loop:
    Identity<Self::I> + Invertibility<Self::I> + Cancellativity<Self::I> + From<Self::I>
{
    type I: Clone + PartialEq;
    fn get(self) -> Self::I;
}

impl<L: Loop> QuasiGroup for L {
    type I = <L as Loop>::I;

    fn get(self) -> Self::I {
        Loop::get(self)
    }
}
