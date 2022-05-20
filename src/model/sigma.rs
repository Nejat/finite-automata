use std::fmt::{self, Debug, Display, Formatter, Write};
use std::ops::Deref;

use crate::utils::duped::Duped;

pub const ERR_DUPLICATE_ALPHABET: &str = "Alphabet must be a unique collection of symbols";
pub const ERR_EMPTY_ALPHABET: &str = "Alphabet must contain at least one symbol";

/// Alphabet of a set of all possible inputs
pub struct Σ<T>(Vec<T>);

impl<T: Eq> Σ<T> {
    /// # Errors
    pub fn new(symbols: Vec<T>) -> Result<Self, &'static str> {
        if symbols.is_empty() {
            Err(ERR_EMPTY_ALPHABET)
        } else if symbols.iter().has_dupes() {
            Err(ERR_DUPLICATE_ALPHABET)
        } else {
            Ok(Self(symbols))
        }
    }
}

impl<T> Deref for Σ<T> {
    type Target = [T];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T: Display> Display for Σ<T> {
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

impl<T: Debug> Debug for Σ<T> {
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
