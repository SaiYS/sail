#![allow(unused_mut, unused_variables)]

#[macro_export]
macro_rules! gen {
    () => {};

    (@rng [$rng:expr] @rest $(,)?) => {};

    (@rng [$rng:expr] @rest, $var:tt = $lower:tt..$upper:tt) => {
        let $var = rand::Rng::gen_range(&mut $rng, $lower, $upper);
    };

    (@rng [$rng:expr] @rest, $var:tt = [$lower:tt..$upper:tt; $rep:tt]) => {
        let $var = (0..$rep).map(|_| rand::Rng::gen_range(&mut $rng, $lower, $upper)).collect::<Vec<_>>();
    };

    (@rng [$rng:expr] @rest, $var:tt = $val:tt) => {
        let $var = $val;
    };

    (@rng [$rng:expr] @rest, $var:tt = $lower:tt..$upper:tt $($rest:tt)*) => {
        let $var = rand::Rng::gen_range(&mut $rng, $lower, $upper);
        $crate::gen! {
            @rng [$rng]
            @rest $($rest)*
        }
    };

    (@rng [$rng:expr] @rest, $var:tt = [$lower:tt..$upper:tt; $rep:tt], $($rest:tt)*) => {
        let $var = (0..$rep).map(|_| rand::Rng::gen_range(&mut $rng, $lower, $upper)).collect::<Vec<_>>();
        $crate::gen! {
            @rng [$rng]
            @rest $($rest)*
        }
    };

    (@rng [$rng:expr] @rest, $var:tt = $val:tt $($rest:tt)*) => {
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
            @rest, $($rest)*
        }
        drop(rng);
    }
}
