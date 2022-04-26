pub trait Operation<I: Clone> {
    fn operate(x: I, y: I) -> I;
    fn operate_assign(x: &mut I, y: I) {
        *x = Self::operate(x.clone(), y);
    }
}

/// `x * (y * z) == (x * y) * z`
pub trait Associativity<I: PartialEq + Clone>: Operation<I> {
    fn check_associative(x: I, y: I, z: I) {
        assert!(
            Self::operate(x.clone(), Self::operate(y.clone(), z.clone()))
                == Self::operate(Self::operate(x, y), z)
        );
    }
}

/// `x * y == y * x`
pub trait Commutativity<I: PartialEq + Clone>: Operation<I> {
    fn check_commutative(x: I, y: I) {
        assert!(Self::operate(x.clone(), y.clone()) == Self::operate(y, x));
    }
}

/// `x * e == x`
pub trait Identity<I: PartialEq + Clone>: Operation<I> {
    fn identity() -> I;
    fn check_identity(x: I) {
        assert!(x == Self::operate(x.clone(), Self::identity()))
    }
}

/// `x * x^1 == e`
pub trait Invertibility<I: PartialEq + Clone>: Operation<I> + Identity<I> {
    fn inverse(x: I) -> I;
    fn check_invertibility(x: I) {
        assert!(Self::operate(x.clone(), Self::inverse(x)) == Self::identity());
        // Here is another wider definition of invertibility without identity
        // assert!(x.clone().operate(y.clone()).operate(y.invertibility()) == x);
    }
}

/// `x * z == y * z => x == y`
///
/// `z * x == z * y => x == y`
pub trait Cancellativity<I: PartialEq + Clone>: Operation<I> {
    fn check_cancellativity(x: I, y: I, z: I) {
        assert!(
            (Self::operate(x.clone(), z.clone()) == Self::operate(y.clone(), z.clone()))
                == (x == y)
                && (Self::operate(z.clone(), x.clone()) == Self::operate(z, y.clone())) == (x == y)
        );
    }
}

/// x * x == x
pub trait Idempotent<I: PartialEq + Clone>: Operation<I> {
    fn check_idempotent(x: I) {
        assert!(Self::operate(x.clone(), x.clone()) == x);
    }
}
