use std::collections::HashSet;
use std::hash::Hash;
use std::ops::Deref;

use crate::utils::duped::Duped;

pub const ERR_DUPLICATE_FINAL_STATES: &str = "Final states must be a unique collection of states";
pub const ERR_EMPTY_FINAL_STATES: &str = "Final states must contain at least one state";

/// Set of final states
pub struct F<S: Eq + Hash>(HashSet<Vec<S>>);

impl<S: Eq + Hash> F<S> {
    /// # Errors
    pub fn new(final_states: Vec<S>) -> Result<Self, &'static str> {
        if final_states.is_empty() {
            Err(ERR_EMPTY_FINAL_STATES)
        } else if final_states.iter().has_dupes() {
            Err(ERR_DUPLICATE_FINAL_STATES)
        } else {
            Ok(Self(final_states.into_iter().map(|s| vec![s]).collect::<HashSet<_>>()))
        }
    }
}

impl<S: Eq + Hash> Deref for F<S> {
    type Target = HashSet<Vec<S>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
