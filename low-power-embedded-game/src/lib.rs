// This stub file contains items that aren't used yet; feel free to remove this module attribute
// to enable stricter warnings.
#![allow(unused)]

pub fn divmod(dividend: i16, divisor: i16) -> (i16, i16) {
    let quotient = dividend / divisor;
    let rest = dividend % divisor;

    (quotient, rest)
}

pub fn evens<T>(iter: impl Iterator<Item = T>) -> impl Iterator<Item = T> {
    iter.enumerate()
        .filter(|&(index, _)| index % 2 == 0)
        .map(|(_, e)| e)
}

pub struct Position(pub i16, pub i16);

impl Position {
    pub fn manhattan(&self) -> i16 {
        let origin = Position(0, 0);

        (origin.0 - self.0).abs() + (origin.1 - self.1).abs()
    }
}
