use crate::youve_been_duped;

pub struct Alphabet<'a, T>(&'a [T]);

pub(crate) const ERR_DUPED_ALPHABET: &str = "Alphabet must be a unique collection of symbols";

impl<'a, T> AsRef<[T]> for Alphabet<'a, T> {
    fn as_ref(&self) -> &[T] {
        self.0
    }
}

impl<'a, T: Eq> Alphabet<'a, T> {
    pub fn new(symbols: &'a [T]) -> Result<Self, &'static str> {
        if youve_been_duped(symbols) {
            Err(ERR_DUPED_ALPHABET)
        } else {
            Ok(Self(symbols))
        }
    }
}
