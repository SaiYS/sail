#[macro_export]
macro_rules! gen {
    (@rng [$rng:expr]) => {};

    // excluded array
    (@rng [$rng:expr] $name:tt = [$lower:tt..$upper:tt; $rep:tt]) => {
        let $name = (0..$rep).map(|_| rand::Rng::gen_range($rng, $lower, $upper)).collect::<Vec<_>>();
    };
    (@rng [$rng:expr] $name:tt = [$lower:tt..$upper:tt; $rep:tt], $($rest:tt)*) => {
        let $name = (0..$rep).map(|_| rand::Rng::gen_range($rng, $lower, $upper)).collect::<Vec<_>>();
        gen! {
            @rng [$rng]
            $($rest)*
        }
    };
    (@rng [$rng:expr] mut $name:tt = [$lower:tt..$upper:tt; $rep:tt]) => {
        let mut $name = (0..$rep).map(|_| rand::Rng::gen_range($rng, $lower, $upper)).collect::<Vec<_>>();
    };
    (@rng [$rng:expr] mut $name:tt = [$lower:tt..$upper:tt; $rep:tt], $($rest:tt)*) => {
        let mut $name = (0..$rep).map(|_| rand::Rng::gen_range($rng, $lower, $upper)).collect::<Vec<_>>();
        gen! {
            @rng [$rng]
            $($rest)*
        }
    };

    // included array
    (@rng [$rng:expr] $name:tt = [$lower:tt..=$upper:tt; $rep:tt]) => {
        let $name = (0..=$rep).map(|_| rand::Rng::gen_range($rng, $lower, $upper + 1)).collect::<Vec<_>>();
    };
    (@rng [$rng:expr] $name:tt = [$lower:tt..=$upper:tt; $rep:tt], $($rest:tt)*) => {
        let $name = (0..$rep).map(|_| rand::Rng::gen_range($rng, $lower, $upper + 1)).collect::<Vec<_>>();
        gen! {
            @rng [$rng]
            $($rest)*
        }
    };
    (@rng [$rng:expr] mut $name:tt = [$lower:tt..=$upper:tt; $rep:tt]) => {
        let mut $name = (0..$rep).map(|_| rand::Rng::gen_range($rng, $lower, $upper + 1)).collect::<Vec<_>>();
    };
    (@rng [$rng:expr] mut $name:tt = [$lower:tt..=$upper:tt; $rep:tt], $($rest:tt)*) => {
        let mut $name = (0..$rep).map(|_| rand::Rng::gen_range($rng, $lower, $upper + 1)).collect::<Vec<_>>();
        gen! {
            @rng [$rng]
            $($rest)*
        }
    };

    // sorted array
    (@rng [$rng:expr] $name:tt = [$lower:tt => $upper:tt; $rep:tt]) => {
        let $name = itertools::Itertools::sorted((0..$rep).map(|_| rand::Rng::gen_range($rng, $lower, $upper + 1))).collect::<Vec<_>>();
    };
    (@rng [$rng:expr] $name:tt = [$lower:tt => $upper:tt; $rep:tt], $($rest:tt)*) => {
        let $name = itertools::Itertools::sorted((0..$rep).map(|_| rand::Rng::gen_range($rng, $lower, $upper + 1))).collect::<Vec<_>>();
        gen! {
            @rng [$rng]
            $($rest)*
        }
    };
    (@rng [$rng:expr] mut $name:tt = [$lower:tt => $upper:tt; $rep:tt]) => {
        let mut $name = itertools::Itertools::sorted((0..$rep).map(|_| rand::Rng::gen_range($rng, $lower, $upper + 1))).collect::<Vec<_>>();
    };
    (@rng [$rng:expr] mut $name:tt = [$lower:tt => $upper:tt; $rep:tt], $($rest:tt)*) => {
        let mut $name = itertools::Itertools::sorted((0..$rep).map(|_| rand::Rng::gen_range($rng, $lower, $upper + 1))).collect::<Vec<_>>();
        gen! {
            @rng [$rng]
            $($rest)*
        }
    };

    // permutation
    (@rng [$rng:expr] $name:tt = [$lower:tt -> $upper:tt; $rep:tt]) => {
        let mut $name = rand::seq::IteratorRandom::choose_multiple($lower..$upper + 1, $rng, $rep);
        rand::seq::SliceRandom::shuffle(AsMut::<[_]>::as_mut(&mut $name), $rng);
        let $name = $name;
    };
    (@rng [$rng:expr] $name:tt = [$lower:tt -> $upper:tt; $rep:tt], $($rest:tt)*) => {
        let mut $name = rand::seq::IteratorRandom::choose_multiple($lower..$upper + 1, $rng, $rep);
        rand::seq::SliceRandom::shuffle(AsMut::<[_]>::as_mut(&mut $name), $rng);
        let $name = $name;
        gen! {
            @rng [$rng]
            $($rest)*
        }
    };
    (@rng [$rng:expr] mut $name:tt = [$lower:tt -> $upper:tt; $rep:tt]) => {
        let mut $name = rand::seq::IteratorRandom::choose_multiple($lower..$upper + 1, $rng, $rep);
        rand::seq::SliceRandom::shuffle(AsMut::<[_]>::as_mut(&mut $name), $rng);
    };
    (@rng [$rng:expr] mut $name:tt = [$lower:tt -> $upper:tt; $rep:tt], $($rest:tt)*) => {
        let mut $name = rand::seq::IteratorRandom::choose_multiple($lower..$upper + 1, $rng, $rep);
        rand::seq::SliceRandom::shuffle(AsMut::<[_]>::as_mut(&mut $name), $rng);
        gen! {
            @rng [$rng]
            $($rest)*
        }
    };

    // fixed value
    (@rng [$rng:expr] $name:tt = $val:tt) => {
        let $name = $val;
    };
    (@rng [$rng:expr] $name:tt = $val:tt, $($rest:tt)*) => {
        let $name = $val;
        gen! {
            @rng [$rng]
            $($rest)*
        }
    };
    (@rng mut [$rng:expr] mut $name:tt = $val:tt) => {
        let mut $name = $val;
    };
    (@rng [$rng:expr] mut $name:tt = $val:tt, $($rest:tt)*) => {
        let mut $name = $val;
        gen! {
            @rng [$rng]
            $($rest)*
        }
    };

    // excluded range
    (@rng [$rng:expr] $name:tt = $lower:tt..$upper:tt) => {
        let $name = rand::Rng::gen_range($rng, $lower, $upper);
    };
    (@rng [$rng:expr] $name:tt = $lower:tt..$upper:tt, $($rest:tt)*) => {
        let $name = rand::Rng::gen_range($rng, $lower, $upper);
        gen! {
            @rng [$rng]
            $($rest)*
        }
    };
    (@rng [$rng:expr] mut $name:tt = $lower:tt..$upper:tt) => {
        let mut $name = rand::Rng::gen_range($rng, $lower, $upper);
    };
    (@rng [$rng:expr] mut $name:tt = $lower:tt..$upper:tt, $($rest:tt)*) => {
        let mut $name = rand::Rng::gen_range($rng, $lower, $upper);
        gen! {
            @rng [$rng]
            $($rest)*
        }
    };

    // included range
    (@rng [$rng:expr] $name:tt = $lower:tt..=$upper:tt) => {
        let $name = rand::Rng::gen_range($rng, $lower, $upper + 1);
    };
    (@rng [$rng:expr] $name:tt = $lower:tt..=$upper:tt, $($rest:tt)*) => {
        let $name = rand::Rng::gen_range($rng, $lower, $upper + 1);
        gen! {
            @rng [$rng]
            $($rest)*
        }
    };
    (@rng [$rng:expr] mut $name:tt = $lower:tt..=$upper:tt) => {
        let mut $name = rand::Rng::gen_range($rng, $lower, $upper + 1);
    };
    (@rng [$rng:expr] mut $name:tt = $lower:tt..=$upper:tt, $($rest:tt)*) => {
        let mut $name = rand::Rng::gen_range($rng, $lower, $upper + 1);
        gen! {
            @rng [$rng]
            $($rest)*
        }
    };

    // initialize rng
    ($($rest:tt)+) => {
        let mut rng = rand::thread_rng();
        gen! {
            @rng [&mut rng]
            $($rest)+
        }
        drop(rng);
    };
}
