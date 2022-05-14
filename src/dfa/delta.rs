use std::collections::HashMap;
use std::fmt::{self, Debug, Display, Formatter};
use std::hash::Hash;

use crate::model::sigma::Σ;
use crate::model::state::{Q, State};
use crate::youve_been_duped;

pub type TransitionState<'a, S> = &'a State<'a, S>;

type Transitions<'a, A, S> = HashMap<&'a State<'a, S>, HashMap<A, &'a State<'a, S>>>;

pub const ERR_DANGLING_STATE: &str = "List of state transitions has dangling states";
pub const ERR_DUPED_TRANSITION: &str = "List of state transitions must be unique";
pub const ERR_UNDEFINED_SYMBOL: &str = "Use of undefined symbol in input transitions";
pub const ERR_UNDEFINED_TRANSITION_STATE: &str = "State transition not in States";
pub const ERR_DUPED_INPUT_TRANSITION: &str = "Each state transition must contain unique inputs";
pub const ERR_INCOMPLETE_INPUT_TRANSITIONS: &str = "Each state must define a transition for all inputs";
pub const ERR_MISSING_FINAL_STATE_TRANSITION: &str = "Transitions Table requires a Final state";
pub const ERR_MISSING_INITIAL_STATE_TRANSITION: &str = "Transitions Table requires an Initial state";
pub const ERR_MISSING_STATE_TRANSITION: &str = "Not all transitions match states in the transitions table";
pub const ERR_REDEFINED_INPUT_TRANSITION: &str = "Each state transition must define each input only once";

const EXPECTED_INITIAL_STATE: &str = "DFA expects an initial state defined in transitions table";

///
#[allow(non_camel_case_types)]
pub struct δ<'a, A: Eq, S: Eq + Hash>(Transitions<'a, A, S>);

impl<'a, A: Eq, S: Eq + Hash> AsRef<Transitions<'a, A, S>> for δ<'a, A, S> {
    fn as_ref(&self) -> &Transitions<'a, A, S> {
        &self.0
    }
}

impl<'a, A: Eq + Hash, S: Eq + Hash> δ<'a, A, S> {
    /// # Errors
    #[allow(non_snake_case)]
    #[allow(clippy::needless_pass_by_value)]
    pub fn new(
        Q: &'a mut Q<'a, S>,
        sigma: Σ<A>,
        delta: Vec<(S, Vec<(A, S)>)>,
        q0: S,
        F: Vec<S>,
    ) -> Result<Self, &'static str> {
        Q.set_phases(&q0, &F)?;

        let mut state_transistion = HashMap::new();

        for (state, input_transitions) in delta {
            let state_node = Q.get_state(&[&state]).ok_or(ERR_UNDEFINED_TRANSITION_STATE)?;

            if state_transistion.contains_key(state_node) {
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
                    Ok((sym, Q.get_state(&[&state]).ok_or(ERR_UNDEFINED_TRANSITION_STATE)?))
                ).collect::<Result<HashMap<_, _>, _>>();

            state_transistion.insert(state_node, inputs?);
        }

        if !state_transistion.keys().any(|state| state.is_initial()) {
            Err(ERR_MISSING_INITIAL_STATE_TRANSITION)
        } else if !state_transistion.keys().any(|state| state.is_final()) {
            Err(ERR_MISSING_FINAL_STATE_TRANSITION)
        } else {
            let states = state_transistion.values().flat_map(HashMap::values);

            if states.clone().all(|state| state_transistion.contains_key(*state)) {
                let transition_states = |transition| state_transistion.iter()
                    .filter_map(
                        move |(state, transitions)|
                            if state == transition {
                                None
                            } else {
                                Some(transitions)
                            }
                    ).flat_map(HashMap::values);

                if state_transistion.keys().any(|state|
                    !state.is_initial() &&
                        transition_states(state).all(|transition| transition != state)) {
                    Err(ERR_DANGLING_STATE)
                } else {
                    Ok(Self(state_transistion))
                }
            } else {
                Err(ERR_MISSING_STATE_TRANSITION)
            }
        }
    }

    pub(crate) fn get_initial_state(&self) -> TransitionState<'a, S> {
        self.0.keys()
            .find(|state| state.is_initial())
            .expect(EXPECTED_INITIAL_STATE)
    }
}

impl<'a, A: Debug + Eq, S: Debug + Display + Eq + Hash> Debug for δ<'a, A, S> {
    fn fmt(&self, fmt: &mut Formatter<'_>) -> fmt::Result {
        fmt.debug_struct("δ")
            .field("δ", &self.0)
            .finish()
    }
}

impl<'a, A: Debug + Display + Eq, S: Display + Eq + Hash> Display for δ<'a, A, S> {
    fn fmt(&self, fmt: &mut Formatter<'_>) -> fmt::Result {
        fmt.write_fmt(format_args!("δ {{ δ: {:?}}}", self.0))
    }
}
