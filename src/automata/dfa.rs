use std::collections::HashMap;
use std::fmt::{self, Debug, Formatter};
use std::hash::Hash;

use crate::automata::{
    convert_to_transitions, ERR_DANGLING_STATE, ERR_INVALID_INPUT,
    get_initial_state, Transitions, validate_fa_configuration,
};
use crate::model::{F, δ};
use crate::model::state::{Q, State};
use crate::model::Σ;
use crate::UNREACHABLE_ERR;

pub const ERR_DUPLICATE_INPUT_TRANSITION: &str = "Each state transition must define unique input transitions";
pub const ERR_INCOMPLETE_INPUT_TRANSITIONS: &str = "Each transition function must define a transition state for all inputs";
pub const ERR_UNDEFINED_SYMBOL: &str = "Symbol is not defined in input transitions";

///
#[allow(clippy::upper_case_acronyms)]
pub struct DFA<A, S: Hash> {
    current: State<S>,
    transitions: Transitions<A, S>,
}

impl<A: Eq + Hash, S: Eq + Hash> DFA<A, S> {
    /// # Errors
    #[allow(non_snake_case)]
    pub fn new(Q: Q<S>, Σ: &Σ<A>, δ: δ<A, S>, q0: S, F: &F<S>) -> Result<Self, &'static str> {
        validate_fa_configuration(&Q, &δ, &q0, F)?;

        let mut transitions = convert_to_transitions(Q, q0, F, &δ)?;

        Self::add_input_transitions(&mut transitions, Σ, δ)?;

        Self::validate_dfa(&transitions)?;

        Ok(Self {
            current: get_initial_state(&transitions),
            transitions,
        })
    }

    ///
    #[must_use]
    pub const fn matches(&self) -> bool {
        self.current.is_final()
    }

    /// # Errors
    pub fn reset(&mut self) {
        self.current = get_initial_state(&self.transitions);
    }

    /// # Errors
    #[allow(clippy::missing_panics_doc)] // see comments in method
    pub fn step(&mut self, input: &A) -> Result<&State<S>, &'static str> {
        let (_, transitions) = self.transitions.get_key_value(&self.current).expect(UNREACHABLE_ERR);

        let next = transitions.get(input).ok_or(ERR_INVALID_INPUT)?;

        let (current, _) = self.transitions.get_key_value(next).expect(UNREACHABLE_ERR);

        self.current = current.clone();

        Ok(&self.current)
    }

    /// # Errors
    pub fn steps(&mut self, inputs: &[A]) -> Result<&State<S>, &'static str> {
        for input in inputs {
            self.step(input)?;
        }

        Ok(&self.current)
    }

    #[inline]
    #[allow(non_snake_case)]
    fn add_input_transitions(
        state_transitions: &mut Transitions<A, S>, Σ: &Σ<A>, δ: δ<A, S>,
    ) -> Result<(), &'static str> {
        for (state, input_transitions) in δ {
            let state_key = vec![state];

            if Σ.iter()
                .any(|sym1| !input_transitions.iter().any(|(sym2, _)| sym2 == sym1)) {
                return Err(ERR_INCOMPLETE_INPUT_TRANSITIONS);
            } else if input_transitions.iter().any(|(sym, _)| !Σ.contains(sym)) {
                return Err(ERR_UNDEFINED_SYMBOL);
            } else if Σ.iter().count() != input_transitions.len() {
                return Err(ERR_DUPLICATE_INPUT_TRANSITION);
            }

            let inputs = input_transitions.into_iter()
                .map(|(sym, state)| (sym, vec![state]))
                .collect::<HashMap<A, Vec<S>>>();

            let entry = state_transitions.get_mut(&state_key).expect(UNREACHABLE_ERR);

            entry.extend(inputs);
        }

        Ok(())
    }

    #[inline]
    fn validate_dfa(state_transitions: &Transitions<A, S>) -> Result<(), &'static str> {
        let transition_states = |transition_state| state_transitions.iter()
            .filter_map(
                move |(state, input_transitions)|
                    if state == transition_state {
                        None
                    } else {
                        Some(input_transitions)
                    }
            ).flat_map(HashMap::values);

        if state_transitions.keys().any(|state| {
            !state.is_initial() &&
                transition_states(state).all(|input_transition| input_transition != state.as_ref())
        }) {
            Err(ERR_DANGLING_STATE)
        } else {
            Ok(())
        }
    }
}

impl<A: Debug, S: Debug + Hash> Debug for DFA<A, S> {
    fn fmt(&self, fmt: &mut Formatter<'_>) -> fmt::Result {
        fmt.debug_struct("DFA")
            .field("δ", &self.transitions)
            .field("current", &self.current)
            .finish()
    }
}
