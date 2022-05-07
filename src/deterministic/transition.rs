use std::collections::HashMap;
use std::hash::Hash;

use crate::deterministic::alphabet::Alphabet;
use crate::deterministic::state::{State, StateNode, States};
use crate::youve_been_duped;

pub(crate) const ERR_DUPED_TRANSITION: &str = "List of state transitions must be unique";
pub(crate) const ERR_UNDEFINED_TRANSITION_STATE: &str = "State transition not in States";
pub(crate) const ERR_DUPED_INPUT_TRANSITION: &str = "Each state transition must contain unique inputs";
pub(crate) const ERR_INCOMPLETE_INPUT_TRANSITIONS: &str = "Each state must define a transition for all inputs";
pub(crate) const ERR_MISSING_STATE_TRANSITION: &str = "Not all States have a transition defined";
pub(crate) const ERR_REDEFINED_INPUT_TRANSITION: &str = "Each state transition must define each input only once";

pub struct TransitionTable<'a, A: Eq, S: Eq + Hash>(HashMap<&'a StateNode<State<'a, S>>, HashMap<A, &'a StateNode<State<'a, S>>>>);

impl<'a, A: Eq + Hash, S: Eq + Hash> TransitionTable<'a, A, S> {
    pub fn new(
        states: &'a States<'a, S>,
        alphabet: &'a Alphabet<'a, A>,
        transitions: Vec<(S, Vec<(A, S)>)>,
    ) -> Result<Self, &'static str> {
        let mut table = HashMap::new();

        for (state, inputs) in transitions {
            let state_node = { states.get_state(state).ok_or(ERR_UNDEFINED_TRANSITION_STATE)? };

            if table.contains_key(state_node) {
                return Err(ERR_DUPED_TRANSITION);
            }

            if youve_been_duped(&inputs) {
                return Err(ERR_DUPED_INPUT_TRANSITION);
            } else if alphabet.as_ref().iter().any(|a| !inputs.iter().any(|v| v.0 == *a)) {
                return Err(ERR_INCOMPLETE_INPUT_TRANSITIONS);
            } else if alphabet.as_ref().iter().count() != inputs.len() {
                return Err(ERR_REDEFINED_INPUT_TRANSITION);
            }

            table.insert(state_node, inputs
                .into_iter()
                .map(|(sym, state)| Ok((sym, states.get_state(state).ok_or(ERR_UNDEFINED_TRANSITION_STATE)?)))
                .collect::<Result<HashMap<_,_>, _>>()?);
        }

        if states.as_ref().len() != table.len() {
            Err(ERR_MISSING_STATE_TRANSITION)
        } else {
            Ok(Self(table))
        }
    }
}
