use std::borrow::BorrowMut;
use std::collections::{HashMap, HashSet};
use std::fmt::{self, Debug, Display, Formatter};
use std::hash::Hash;

use crate::model::sigma::Σ;
use crate::model::state::{Q, State};

pub type TransitionState<'a, S> = HashSet<&'a State<'a, S>>;

type Transitions<'a, A, S> = HashMap<&'a State<'a, S>, HashMap<A, TransitionState<'a, S>>>;

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
    #[allow(non_snake_case)]
    #[allow(clippy::needless_pass_by_value)]
    pub fn new(
        Q: &'a mut Q<'a, S>,
        sigma: &'a Σ<'a, A>,
        delta: Vec<(S, Vec<(A, S)>)>,
        q0: S,
        F: Vec<S>,
    ) -> Result<Self, &'static str> {
        Q.borrow_mut().set_phases(&q0, &F)?;

        let mut state_transitions = HashMap::new();

        for (state, input_transitions) in delta {
            let state_node = Q.get_state(&[&state]).ok_or(ERR_UNDEFINED_TRANSITION_STATE)?;

            if state_transitions.contains_key(state_node) {
                return Err(ERR_DUPED_TRANSITION);
            }

            let inputs = input_transitions.into_iter()
                .fold(
                    Ok(HashMap::new()),
                    |acc, (sym, state)|
                        match acc {
                            Ok(mut acc) => {
                                if sigma.as_ref().contains(&sym) {
                                    let state = Q.get_state(&[&state])
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

            state_transitions.insert(state_node, inputs?);
        }

        if !state_transitions.keys().any(|state| state.is_initial()) {
            Err(ERR_MISSING_INITIAL_STATE_TRANSITION)
        } else if !state_transitions.keys().any(|state| state.is_final()) {
            Err(ERR_MISSING_FINAL_STATE_TRANSITION)
        } else {
            let states = state_transitions.values().flat_map(HashMap::values);

            if states.clone().all(|states| states.iter().all(|state| state_transitions.contains_key(*state))) {
                let transition_states = |transition| state_transitions.iter()
                    .filter_map(
                        move |(state, transitions)|
                            if state == transition {
                                None
                            } else {
                                Some(transitions)
                            }
                    ).flat_map(HashMap::values);

                if state_transitions.keys().any(
                    |state| !state.is_initial() &&
                        transition_states(state).all(
                            |states| states.iter().all(|transition| transition != state)
                        )
                ) {
                    Err(ERR_DANGLING_STATE)
                } else {
                    Ok(Self {
                        sigma,
                        delta: state_transitions,
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
                .find(|state| state.is_initial())
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
