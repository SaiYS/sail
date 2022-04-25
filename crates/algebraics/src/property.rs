use super::Operation;

/// `x * (y * z) == (x * y) * z`
pub trait Associativity: Operation + PartialEq {
    fn check_associative(x: Self, y: Self, z: Self) {
        assert!(x.clone().operate(y.clone()).operate(z.clone()) == (x.operate(y)).operate(z));
    }
}

/// `x * y == y * x`
pub trait Commutativity: Operation + PartialEq {
    fn check_commutative(x: Self, y: Self) {
        assert!(x.clone().operate(y.clone()) == y.operate(x));
    }
}

/// `x * e == x`
pub trait Identity: Operation + PartialEq {
    fn identity() -> Self;
    fn check_identity(x: Self) {
        assert!(x.clone() == x.operate(Self::identity()))
    }
}

/// `x * x^1 == e`
pub trait Invertibility: Operation + Identity + PartialEq {
    fn inverse(self) -> Self;
    fn check_invertibility(x: Self) {
        assert!(x.clone().operate(x.inverse()) == Self::identity());
        // Here is another wider definition of invertibility without identity
        // assert!(x.clone().operate(y.clone()).operate(y.invertibility()) == x);
    }
}

/// `x * z == y * z => x == y`
///
/// `z * x == z * y => x == y`
pub trait Cancellativity: Operation + PartialEq {
    fn check_cancellativity(x: Self, y: Self, z: Self) {
        assert!(
            (x.clone().operate(z.clone()) == y.clone().operate(z.clone())) == (x == y)
                && (z.clone().operate(x.clone()) == z.operate(y.clone())) == (x == y)
        );
    }
}

/// x * x == x
pub trait Idempotent: Operation + PartialEq {
    fn check_idempotent(x: Self) {
        assert!(x.clone().operate(x.clone()) == x);
    }
}
