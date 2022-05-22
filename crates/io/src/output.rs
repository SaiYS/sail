pub mod wrapper;

/// Output answer for polar question (yes-no question)
///
/// if `flag` is true, `yes` will print, else `no` will print.
///
/// ```
/// use io::output::util::polar_question;
///
/// polar_question(true, "Alice", "Bob"); // Alice
/// polar_question(false, "Alice", "Bob"); // Bob
/// ```
pub fn polar_question<S: AsRef<str>>(flag: bool, yes: S, no: S) {
    println!("{}", if flag { yes.as_ref() } else { no.as_ref() });
}

pub fn yn(flag: bool) {
    polar_question(flag, "yes", "no");
}

#[allow(non_snake_case)]
pub fn Yn(flag: bool) {
    polar_question(flag, "Yes", "No");
}

#[allow(non_snake_case)]
pub fn YN(flag: bool) {
    polar_question(flag, "YES", "NO");
}

#[macro_export]
macro_rules! vis {
    () => {};
    ($item:expr) => {
        println!("{}", $crate::output::wrapper::VisWrapper($item));
    };
    ($item:expr , $($rest:tt)*) => {
        print!("{} ", $crate::output::wrapper::VisWrapper($item));
        vis!($($rest)*);
    };
    ($item:expr ; $($rest:tt)*) => {
        println!("{}", $crate::output::wrapper::VisWrapper($item));
        vis!($($rest)*);
    };
}

#[macro_export]
macro_rules! dvis {
    ($($item:expr),*) => {
        #[cfg(debug_assertions)]
        dbg!($($item),*)
    };
}
