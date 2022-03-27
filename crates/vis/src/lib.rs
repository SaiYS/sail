#[macro_export]
macro_rules! vis {
    () => {
        println!();
    };
    ($last:expr ;) => {
        print!("{}", $last.lines());
        vis!()
    };
    ($last:expr =>) => {
        print!("{}", $last.continuous());
        vis!();
    };
    ($last:expr $(,)?) => {
        print!("{}", $last.spaces());
        vis!();
    };
    ($first:expr; $($rest:tt)*) => {
        print!("{}", $first.lines());
        println!();
        vis!($($rest)*);
    };
    ($first:expr => $($rest:tt)*) => {
        print!("{}", $first.continuous());
        vis!($($rest)*);
    };
    ($first:expr, $($rest:tt)*) => {
        print!("{}", $first.spaces());
        print!(" ");
        vis!($($rest)*);
    };
}

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

pub mod visualize;
