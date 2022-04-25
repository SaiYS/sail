pub use proconio::{input, source::line::LineSource};

#[macro_export]
macro_rules! interactive_input {
    ($($arg:tt)*) => {
        let mut source = proconio::source::line::LineSource::new(std::io::BufReader::new(std::io::stdin()));
        proconio::input! {
            from &mut source,
            $($arg)*
        }
    };
}

#[test]
fn interactive_input_test() {
    interactive_input! {
        n: usize
    }

    let mut rec = Vec::new();
    for _ in 0..n {
        interactive_input! {
            c: char
        }
        println!("{}", c);
        rec.push(c);
    }

    assert_eq!(rec.len(), n);
}
