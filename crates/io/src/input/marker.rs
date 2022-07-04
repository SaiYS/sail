//! Custom markers for proconio

use crate::proconio::{__Readable as Readable, source::Source};
use std::io::BufRead;

/// Parse a number in 10-radix into Vec of decimal dights.
///
/// This is useful when you want to parse a number
/// that is too big to express in fixd bit-size.
pub enum Digits {}

impl Readable for Digits {
    type Output = Vec<u8>;
    fn read<R: BufRead, S: Source<R>>(source: &mut S) -> Vec<u8> {
        source
            .next_token_unwrap()
            .chars()
            .map(|x| {
                x.to_digit(10).expect(concat!(
                    "failed to parse input to digits",
                    " check if the input number contains some character except 0-9"
                )) as u8
            })
            .collect()
    }
}
