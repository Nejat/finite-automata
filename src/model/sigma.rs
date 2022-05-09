use std::fmt::{self, Debug, Display, Formatter, Write};

use crate::youve_been_duped;

pub(crate) const ERR_DUPED_ALPHABET: &str = "Alphabet must be a unique collection of symbols";

pub struct Σ<'a, T>(&'a [T]);

impl<'a, T: Display> Display for Σ<'a, T> {
    fn fmt(&self, fmt: &mut Formatter<'_>) -> fmt::Result {
        fmt.write_char('[')?;
        for itm in self.0.iter().take(1) {
            fmt.write_fmt(format_args!("{itm}"))?;
        }
        for itm in self.0.iter().skip(1) {
            fmt.write_fmt(format_args!(", {itm}"))?;
        }
        fmt.write_char(']')
    }
}

impl<'a, T: Debug> Debug for Σ<'a, T> {
    fn fmt(&self, fmt: &mut Formatter<'_>) -> fmt::Result {
        fmt.write_char('[')?;
        for itm in self.0.iter().take(1) {
            fmt.write_fmt(format_args!("{itm:?}"))?;
        }
        for itm in self.0.iter().skip(1) {
            fmt.write_fmt(format_args!(", {itm:?}"))?;
        }
        fmt.write_char(']')
    }
}

impl<'a, T> AsRef<[T]> for Σ<'a, T> {
    fn as_ref(&self) -> &[T] {
        self.0
    }
}

impl<'a, T: Eq> Σ<'a, T> {
    pub fn new(symbols: &'a [T]) -> Result<Self, &'static str> {
        if youve_been_duped(symbols) {
            Err(ERR_DUPED_ALPHABET)
        } else {
            Ok(Self(symbols))
        }
    }
}
