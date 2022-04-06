#![allow(unused_mut, unused_variables)]

#[macro_export]
macro_rules! gen {
    () => {};

    (@rng [$rng:expr] @rest $(,)?) => {};

    (@rng [$rng:expr] @rest, $var:ident = $lower:expr => $upper:expr) => {
        let $var = rand::Rng::gen_range(&mut $rng, $lower, $upper);
    };

    (@rng [$rng:expr] @rest, $var:ident = [$lower:expr => $upper:expr; $rep:expr]) => {
        let $var = (0..$rep).map(|_| rand::Rng::gen_range(&mut $rng, $lower, $upper)).collect::<Vec<_>>();
    };

    (@rng [$rng:expr] @rest, $var:ident = $val:expr) => {
        let $var = $val;
    };

    (@rng [$rng:expr] @rest, $var:ident = $lower:tt..$upper:tt $($rest:tt)*) => {
        let $var = rand::Rng::gen_range(&mut $rng, $lower, $upper);
        $crate::gen! {
            @rng [$rng]
            @rest $($rest)*
        }
    };

    (@rng [$rng:expr] @rest, $var:ident = [$lower:tt..$upper:tt; $rep:tt], $($rest:tt)*) => {
        let $var = (0..$rep).map(|_| rand::Rng::gen_range(&mut $rng, $lower, $upper)).collect::<Vec<_>>();
        $crate::gen! {
            @rng [$rng]
            @rest $($rest)*
        }
    };

    (@rng [$rng:expr] @rest, $var:ident = $val:expr, $($rest:tt)*) => {
        let $var = $val;
        $crate::gen! {
            @rng [$rng]
            @rest $($rest)*
        }
    };

    ($($rest:tt)*) => {
        let mut rng = rand::thread_rng();
        $crate::gen! {
            @rng [&mut rng]
            @rest, $($rest),*
        }
        drop(rng);
    }
}

#[test]
fn feature() {
    gen! {
        a = 10,
        b = 1..2,
        c = [3..100; 9]
    }
}
