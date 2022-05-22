use once_cell::sync::Lazy;
use proconio::source::line::LineSource;
use std::{
    io::{BufReader, Stdin},
    sync::Mutex,
};

pub static INTERACTIVE_INPUT_SOURCE: Lazy<Mutex<LineSource<BufReader<Stdin>>>> =
    Lazy::new(|| Mutex::new(LineSource::new(std::io::BufReader::new(std::io::stdin()))));

#[macro_export]
macro_rules! interactive_input {
    ($($arg:tt)*) => {
        let mut __interactive_input_source = $crate::input::INTERACTIVE_INPUT_SOURCE.lock().unwrap();
        eprintln!("locked");
        $crate::proconio::input! {
            from &mut *__interactive_input_source,
            $($arg)*
        }
        drop(__interactive_input_source);
    };
}

#[test]
#[ignore = "need user input"]
fn interactive_input_test() {
    interactive_input! {
        n: usize
    }

    println!("{}", n);

    let mut rec = Vec::new();
    for _ in 0..n {
        interactive_input! {
            c: usize
        }
        println!("{}", c);
        rec.push(c);
    }

    assert_eq!(rec.len(), n);
}
