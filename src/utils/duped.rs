use crate::UNREACHABLE_ERR;

///
pub trait Duped {
    ///
    fn has_dupes(&self) -> bool;
}

impl<'a, T, I> Duped for I
    where T: Eq + 'a,
          I: Clone + Iterator<Item=&'a T> + ExactSizeIterator
{
    fn has_dupes(&self) -> bool {
        (1..self.len()).any(|idx| {
            let check = &self.clone().nth(idx - 1).expect(UNREACHABLE_ERR);

            self.clone().skip(idx).any(|value| value == *check)
        })
    }
}
