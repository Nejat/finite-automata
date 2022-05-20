use std::collections::HashMap;
use std::fmt::{self, Debug, Display, Formatter};
use std::hash::Hash;

use crate::automata::{
    convert_to_transitions, ERR_DANGLING_STATE, ERR_INVALID_INPUT,
    get_initial_state, Transitions, validate_fa_configuration,
};
use crate::model::{F, δ, Σ};
use crate::model::state::{Q, State};
use crate::UNREACHABLE_ERR;

pub const ERR_UNDEFINED_SYMBOL: &str = "Symbol is not defined in input transitions";
pub const ERR_UNDEFINED_TRANSITION_STATE: &str = "State transition not in States";

const EXPECTED_TRANSITION_DEFINED: &str = "NFA expects all transition states defined in transitions table";

///
#[allow(clippy::upper_case_acronyms)]
#[allow(non_snake_case)]
pub struct NFA<A: Eq, S: Eq + Hash> {
    Σ: Σ<A>,
    current: Vec<State<S>>,
    transitions: Transitions<A, S>,
}

impl<A: Eq + Hash, S: Copy + Eq + Hash> NFA<A, S> {
    /// # Errors
    #[allow(non_snake_case)]
    pub fn new(Q: Q<S>, Σ: Σ<A>, δ: δ<A, S>, q0: S, F: &F<S>) -> Result<Self, &'static str> {
        validate_fa_configuration(&Q, &δ, &q0, F)?;

        let mut transitions = convert_to_transitions(Q, q0, F, &δ)?;

        Self::add_input_transitions(&mut transitions, &Σ, δ)?;

        Self::validate_nfa(&transitions)?;

        let nfa = Self {
            Σ,
            current: vec![get_initial_state(&transitions)],
            transitions,
        };

        Ok(nfa)
    }

    ///
    #[must_use]
    pub fn matches(&self) -> bool {
        self.current.iter().any(State::is_final)
    }

    ///
    pub fn reset(&mut self) {
        self.current = vec![get_initial_state(&self.transitions)];
    }

    /// # Errors
    pub fn step(&mut self, input: &A) -> Result<&Vec<State<S>>, &'static str> {
        if !self.Σ.contains(input) {
            return Err(ERR_INVALID_INPUT);
        }

        let transitions = self.current.iter().map(
            |current| self.transitions.get(current).expect(EXPECTED_TRANSITION_DEFINED)
        );

        let current = transitions
            .filter_map(|transitions| transitions.get(input))
            .filter(|next| next.iter().any(|tag| self.transitions.keys().any(|key| key.contains(tag))))
            .fold(
                Ok(Vec::new()),
                |mut acc, next| {
                    if let Ok(ref mut tags) = acc {
                        for tag in next {
                            tags.push(self.transitions.get_key_value(&vec![*tag]).expect(UNREACHABLE_ERR).0.clone());
                        }
                    }

                    acc
                },
            )?;

        self.current = current;

        Ok(&self.current)
    }

    /// # Errors
    pub fn steps(&mut self, inputs: &[A]) -> Result<&Vec<State<S>>, &'static str> {
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

            let inputs = input_transitions.into_iter()
                .fold(
                    Ok(HashMap::new()),
                    |acc, (sym, state)|
                        match acc {
                            Ok(mut acc) => {
                                if Σ.contains(&sym) {
                                    let mut state = vec![state];

                                    if state_transitions.contains_key(&state) {
                                        let state = state.pop().expect(UNREACHABLE_ERR);
                                        let entry = acc.entry(sym).or_insert_with(Vec::new);

                                        if !entry.contains(&state) {
                                            entry.push(state);
                                        }

                                        Ok(acc)
                                    } else {
                                        Err(ERR_UNDEFINED_TRANSITION_STATE)
                                    }
                                } else {
                                    Err(ERR_UNDEFINED_SYMBOL)
                                }
                            }
                            err @ Err(_) => err
                        },
                );

            let entry = state_transitions.get_mut(&state_key).expect(UNREACHABLE_ERR);

            entry.extend(inputs?);
        }

        Ok(())
    }

    #[inline]
    fn validate_nfa(state_transitions: &Transitions<A, S>) -> Result<(), &'static str> {
        let transition_states = |transition_state| state_transitions.iter()
            .filter_map(
                move |(state, input_transitions)|
                    if state == transition_state {
                        None
                    } else {
                        Some(input_transitions.values())
                    }
            )
            .flatten()
            .flatten();

        if state_transitions.keys().any(|state|
            !state.is_initial() &&
                transition_states(state)
                    .all(|input_transition| state.as_ref().iter().all(|s| s != input_transition))
        ) {
            Err(ERR_DANGLING_STATE)
        } else {
            Ok(())
        }
    }
}

impl<A: Debug + Eq, S: Debug + Display + Eq + Hash> Debug for NFA<A, S> {
    fn fmt(&self, fmt: &mut Formatter<'_>) -> fmt::Result {
        fmt.debug_struct("NFA")
            .field("δ", &self.transitions)
            .field("current", &self.current)
            .finish()
    }
}

// todo: output as table
impl<A: Debug + Display + Eq, S: Debug + Display + Eq + Hash> Display for NFA<A, S> {
    fn fmt(&self, fmt: &mut Formatter<'_>) -> fmt::Result {
        fmt.write_fmt(format_args!("δ {{ δ: {:?}}}", self.transitions))
    }
}
