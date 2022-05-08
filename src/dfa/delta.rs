use std::collections::HashMap;
use std::hash::Hash;

use crate::model::sigma::Σ;
use crate::model::state::{Q, State, Tag};
use crate::youve_been_duped;

pub(crate) const ERR_DANGLING_STATE: &str = "List of state transitions has dangling states";
pub(crate) const ERR_DUPED_TRANSITION: &str = "List of state transitions must be unique";
pub(crate) const ERR_UNDEFINED_SYMBOL: &str = "Use of undefined symbol in input transitions";
pub(crate) const ERR_UNDEFINED_TRANSITION_STATE: &str = "State transition not in States";
pub(crate) const ERR_DUPED_INPUT_TRANSITION: &str = "Each state transition must contain unique inputs";
pub(crate) const ERR_INCOMPLETE_INPUT_TRANSITIONS: &str = "Each state must define a transition for all inputs";
pub(crate) const ERR_MISSING_FINAL_STATE_TRANSITION: &str = "Transitions Table requires a Final state";
pub(crate) const ERR_MISSING_INITIAL_STATE_TRANSITION: &str = "Transitions Table requires an Initial state";
pub(crate) const ERR_MISSING_STATE_TRANSITION: &str = "Not all transitions match states in the transitions table";
pub(crate) const ERR_REDEFINED_INPUT_TRANSITION: &str = "Each state transition must define each input only once";

const EXPECTED_INITIAL_STATE: &str = "DFA expects an initial state defined in transitions table";

type Transitions<'a, A, S> = HashMap<&'a State<Tag<'a, S>>, HashMap<A, &'a State<Tag<'a, S>>>>;

#[allow(non_camel_case_types)]
pub struct δ<'a, A: Eq, S: Eq + Hash>(Transitions<'a, A, S>);

impl<'a, A: Eq, S: Eq + Hash> AsRef<Transitions<'a, A, S>> for δ<'a, A, S> {
    fn as_ref(&self) -> &Transitions<'a, A, S> {
        &self.0
    }
}

impl<'a, A: Eq + Hash, S: Eq + Hash> δ<'a, A, S> {
    pub fn new(
        q: &'a Q<'a, S>,
        sigma: &'a Σ<'a, A>,
        delta: Vec<(S, Vec<(A, S)>)>,
    ) -> Result<Self, &'static str> {
        let mut table = HashMap::new();

        for (state, input_transitions) in delta {
            let state_node = { q.get_state(state).ok_or(ERR_UNDEFINED_TRANSITION_STATE)? };

            if table.contains_key(state_node) {
                return Err(ERR_DUPED_TRANSITION);
            }

            if youve_been_duped(&input_transitions) {
                return Err(ERR_DUPED_INPUT_TRANSITION);
            } else if sigma.as_ref().iter().any(|sym1| !input_transitions.iter().any(|(sym2, _)| sym2 == sym1)) {
                return Err(ERR_INCOMPLETE_INPUT_TRANSITIONS);
            } else if input_transitions.iter().any(|(sym, _)| !sigma.as_ref().contains(sym)) {
                return Err(ERR_UNDEFINED_SYMBOL);
            } else if sigma.as_ref().iter().count() != input_transitions.len() {
                return Err(ERR_REDEFINED_INPUT_TRANSITION);
            }

            let inputs = input_transitions.into_iter()
                .map(|(sym, state)|
                    Ok((sym, q.get_state(state).ok_or(ERR_UNDEFINED_TRANSITION_STATE)?))
                ).collect::<Result<HashMap<_, _>, _>>();

            table.insert(state_node, inputs?);
        }

        if !table.keys().any(|key| matches!(key, State::Initial(_))) {
            Err(ERR_MISSING_INITIAL_STATE_TRANSITION)
        } else if !table.keys().any(|key| matches!(key, State::Final(_))) {
            Err(ERR_MISSING_FINAL_STATE_TRANSITION)
        } else {
            let states = table.values().flat_map(|transitions| transitions.values());

            if states.clone().all(|state| table.contains_key(*state)) {
                let transition_states = |state| table.iter()
                    .filter_map(move |(k, v)| if k != state { Some(v) } else { None })
                    .flat_map(|transitions| transitions.values());

                if table.keys().any(|key| !transition_states(key).any(|state| state == key)) {
                    Err(ERR_DANGLING_STATE)
                } else {
                    Ok(Self(table))
                }
            } else {
                Err(ERR_MISSING_STATE_TRANSITION)
            }
        }
    }

    pub(crate) fn get_initial_state(&self) -> &'a State<Tag<'a, S>> {
        self.0.keys()
            .find(|s| matches!(s, State::Initial(_)))
            .expect(EXPECTED_INITIAL_STATE)
    }
}
