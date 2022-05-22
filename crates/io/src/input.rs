use std::{
    io::{BufReader, Stdin},
    sync::Mutex,
};

pub mod marker;

/// Dynamic buffer of stdin for interactive input
pub static __INTERACTIVE_INPUT_SOURCE: once_cell::sync::Lazy<
    Mutex<proconio::source::line::LineSource<BufReader<Stdin>>>,
> = once_cell::sync::Lazy::new(|| {
    Mutex::new(proconio::source::line::LineSource::new(
        std::io::BufReader::new(std::io::stdin()),
    ))
});

/// proconio::input for interactive input
#[macro_export]
macro_rules! input_interactive {
    ($($arg:tt)*) => {
        let mut __interactive_input_source = $crate::input::__INTERACTIVE_INPUT_SOURCE
            .lock()
            .expect("failed to get a lock of stdin");
        $crate::proconio::input! {
            from &mut *__interactive_input_source,
            $($arg)*
        }
        drop(__interactive_input_source);
    };
}
