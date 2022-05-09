use std::collections::{HashMap, HashSet};
use std::fmt::{self, Debug, Display, Formatter};
use std::hash::Hash;

use crate::model::sigma::Σ;
use crate::model::state::{Q, State, Tag};

pub type TransitionState<'a, S> = HashSet<&'a State<Tag<'a, S>>>;

type Transitions<'a, A, S> = HashMap<&'a State<Tag<'a, S>>, HashMap<A, TransitionState<'a, S>>>;

pub const ERR_DANGLING_STATE: &str = "List of state transitions has dangling states";
pub const ERR_DUPED_TRANSITION: &str = "List of state transitions must be unique";
pub const ERR_UNDEFINED_SYMBOL: &str = "Use of undefined symbol in input transitions";
pub const ERR_UNDEFINED_TRANSITION_STATE: &str = "State transition not in States";
pub const ERR_DUPED_INPUT_TRANSITION: &str = "Each state transition must contain unique inputs";
pub const ERR_MISSING_FINAL_STATE_TRANSITION: &str = "Transitions Table requires a Final state";
pub const ERR_MISSING_INITIAL_STATE_TRANSITION: &str = "Transitions Table requires an Initial state";
pub const ERR_MISSING_STATE_TRANSITION: &str = "Not all transitions match states in the transitions table";

const EXPECTED_INITIAL_STATE: &str = "DFA expects an initial state defined in transitions table";

///
#[allow(non_camel_case_types)]
pub struct δ<'a, A: Eq, S: Eq + Hash> {
    sigma: &'a Σ<'a, A>,
    delta: Transitions<'a, A, S>,
}

impl<'a, A: Eq, S: Eq + Hash> AsRef<Transitions<'a, A, S>> for δ<'a, A, S> {
    fn as_ref(&self) -> &Transitions<'a, A, S> {
        &self.delta
    }
}

impl<'a, A: Eq + Hash, S: Eq + Hash> δ<'a, A, S> {
    /// # Errors
    pub fn new(
        q: &'a Q<'a, S>,
        sigma: &'a Σ<'a, A>,
        delta_source: Vec<(S, Vec<(A, S)>)>,
    ) -> Result<Self, &'static str> {
        let mut delta = HashMap::new();

        for (state, input_transitions) in delta_source {
            let state_node = { q.get_state(&state).ok_or(ERR_UNDEFINED_TRANSITION_STATE)? };

            if delta.contains_key(state_node) {
                return Err(ERR_DUPED_TRANSITION);
            }

            let inputs = input_transitions.into_iter()
                .fold(
                    Ok(HashMap::new()),
                    |acc, (sym, state)|
                        match acc {
                            Ok(mut acc) => {
                                if sigma.as_ref().contains(&sym) {
                                    let state = q.get_state(&state)
                                        .ok_or(ERR_UNDEFINED_TRANSITION_STATE)?;

                                    let entry = acc.entry(sym).or_insert_with(HashSet::new);

                                    if entry.contains(&state) {
                                        Err(ERR_DUPED_INPUT_TRANSITION)
                                    } else {
                                        entry.insert(state);

                                        Ok(acc)
                                    }
                                } else {
                                    Err(ERR_UNDEFINED_SYMBOL)
                                }
                            }
                            err @ Err(_) => err
                        },
                );

            delta.insert(state_node, inputs?);
        }

        if !delta.keys().any(|key| matches!(key, State::Initial(_))) {
            Err(ERR_MISSING_INITIAL_STATE_TRANSITION)
        } else if !delta.keys().any(|key| matches!(key, State::Final(_))) {
            Err(ERR_MISSING_FINAL_STATE_TRANSITION)
        } else {
            let states = delta.values().flat_map(HashMap::values);

            if states.clone().all(|states| states.iter().all(|state| delta.contains_key(*state))) {
                let transition_states = |transition| delta.iter()
                    .filter_map(
                        move |(state, transitions)|
                            if state == transition {
                                None
                            } else {
                                Some(transitions)
                            }
                    ).flat_map(HashMap::values);

                if delta.keys().any(|state|
                    !matches!(state, State::Initial(_)) &&
                        transition_states(state).all(
                            |states| states.iter().all(|transition| transition != state)
                        )
                ) {
                    Err(ERR_DANGLING_STATE)
                } else {
                    Ok(Self {
                        sigma,
                        delta,
                    })
                }
            } else {
                Err(ERR_MISSING_STATE_TRANSITION)
            }
        }
    }

    pub(crate) fn get_initial_state(&self) -> TransitionState<'a, S> {
        let mut state = HashSet::new();

        state.insert(
            *self.delta.keys()
                .find(|s| matches!(s, State::Initial(_)))
                .expect(EXPECTED_INITIAL_STATE)
        );

        state
    }

    pub(crate) fn is_valid_input(&self, input: &A) -> bool {
        self.sigma.as_ref().contains(input)
    }
}

impl<'a, A: Debug + Eq, S: Debug + Display + Eq + Hash> Debug for δ<'a, A, S> {
    fn fmt(&self, fmt: &mut Formatter<'_>) -> fmt::Result {
        fmt.debug_struct("δ")
            .field("Σ", &self.sigma)
            .field("δ", &self.delta)
            .finish()
    }
}

impl<'a, A: Debug + Display + Eq, S: Display + Eq + Hash> Display for δ<'a, A, S> {
    fn fmt(&self, fmt: &mut Formatter<'_>) -> fmt::Result {
        fmt.write_fmt(format_args!("δ {{ Σ: {},  δ: {:?}}}", self.sigma, self.delta))
    }
}
