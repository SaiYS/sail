#[macro_export]
macro_rules! yn {
    ($($flag:expr),*) => {
        $(
            println!("{}", if $flag { "yes" } else { "no" });
        )*
    };
}

#[macro_export]
macro_rules! Yn {
    ($($flag:expr),*) => {
        $(
            println!("{}", if $flag { "Yes" } else { "No" });
        )*
    };
}

#[macro_export]
macro_rules! YN {
    ($($flag:expr),*) => {
        $(
            println!("{}", if $flag { "YES" } else { "NO" });
        )*
    };
}

pub mod output;
pub mod input;