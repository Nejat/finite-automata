#![deny(clippy::all)]
#![deny(clippy::pedantic)]
#![deny(clippy::nursery)]
#![deny(clippy::cargo)]
#![deny(missing_docs)]
// ==============================================================
#![allow(clippy::module_name_repetitions)]
#![allow(clippy::items_after_statements)]
// ==============================================================
#![doc(html_root_url = "https://docs.rs/finite-state-machine/0.1.0")]

//!

pub mod dfa;
pub mod model;
pub mod nfa;

#[cfg(test)]
mod tests;

/*trait Duped<T: Eq> {
   fn youve_been_duped(&self) -> bool;
}

impl<T: Eq> Duped<T> for &[T] {
    fn youve_been_duped(&self) -> bool {
        (1..self.len()).any(|i| self[i..].contains(&self[i - 1]))
    }
}*/

/*impl<T: Eq, R: AsRef<T>> Duped<R> for &[R] {
    fn youve_been_duped(&self) -> bool {
        (1..self.len()).any(|i| self[i..].contains(&self[i - 1]))
    }
}
*/

fn youve_been_duped<T: Eq>(values: &[T]) -> bool {
    (1..values.len()).any(|i| values[i..].contains(&values[i - 1]))
}

/*fn youve_been_duped_ref<T: Eq, R: AsRef<T>>(values: &[R]) -> bool {
    (1..values.len()).any(|i| values[i..].iter().map(AsRef::as_ref).any(|v| v == values[i - 1].as_ref()))
}
*/