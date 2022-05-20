use std::ops::Deref;

use crate::utils::duped::Duped;

pub const ERR_DUPLICATE_DELTA_STATES: &str = "Transition functions must be a unique collection of state transitions";
pub const ERR_DUPLICATE_INPUT_TRANSITIONS: &str = "A state transition must must be a unique collection of input transitions";
pub const ERR_UNDEFINED_STATE_TRANSITION: &str = "Input transition state does not correspond to a state transition";

#[allow(non_camel_case_types)]
type delta<A, S> = (S, Vec<(A, S)>);
type Delta<A, S> = Vec<delta<A, S>>;

/// Transition function Q X Σ -> Q
#[allow(non_camel_case_types)]
pub struct δ<A, S>(Delta<A, S>);

impl<A: Eq, S: Eq> δ<A, S> {
    /// # Errors
    pub fn new(delta: Delta<A, S>) -> Result<Self, &'static str> {
        let states = delta.iter().map(|(state, _)| state);

        if states.has_dupes() {
            Err(ERR_DUPLICATE_DELTA_STATES)
        } else if delta.iter().any(|(_, transitions)| transitions.iter().has_dupes()) {
            Err(ERR_DUPLICATE_INPUT_TRANSITIONS)
        } else {
            let state_transitions = delta.iter()
                .flat_map(|(_, transitions)|
                    transitions.iter().map(|(_, state)| state)
                );

            if state_transitions.clone().all(|transition|
                states.clone().any(|state| transition == state)
            ) {
                Ok(Self(delta))
            } else {
                Err(ERR_UNDEFINED_STATE_TRANSITION)
            }
        }
    }

    pub(crate) fn states_transitioned(&self) -> impl Iterator<Item=&S> {
        self.0.iter().flat_map(|itm| itm.1.iter().map(|itm| &itm.1))
    }
}

impl<A, S> Deref for δ<A, S> {
    type Target = Delta<A, S>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<A, S> IntoIterator for δ<A, S> {
    type Item = delta<A, S>;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}
