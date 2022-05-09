use std::collections::HashSet;
use std::hash::Hash;

use crate::model::state::State;
use crate::nfa::delta::{TransitionState, δ};

pub(crate) const ERR_INVALID_INPUT: &str = "Invalid input encountered";

const EXPECTED_TRANSITION_DEFINED: &str = "NFA expects all transition states defined in transitions table";

#[allow(clippy::upper_case_acronyms)]
pub struct NFA<'a, A: Eq, S: Eq + Hash> {
    delta: δ<'a, A, S>,
    current: TransitionState<'a, S>,
}

impl<'a, A: Eq + Hash, S: Eq + Hash> NFA<'a, A, S>
{
    pub fn new(delta: δ<'a, A, S>) -> Self {
        let current = delta.get_initial_state();

        Self {
            current,
            delta,
        }
    }

    pub fn matches(&self) -> bool {
        self.current.iter().any(|state| matches!(state, State::Final(_)))
    }

    pub fn reset(&mut self) {
        self.current = self.delta.get_initial_state();
    }

    pub fn step(&mut self, input: &A) -> Result<&TransitionState<'a, S>, &'static str> {
        if !self.delta.is_valid_input(input) {
            return Err(ERR_INVALID_INPUT);
        }

        let transitions = self.current.iter().map(
            |current| self.delta.as_ref().get(current).expect(EXPECTED_TRANSITION_DEFINED)
        );

        let current = transitions
            .filter_map(|transitions| transitions.get(input))
            .flatten()
            .copied()
            .collect::<HashSet<_>>();

        self.current = current;

        Ok(&self.current)
    }

    pub fn steps(&mut self, inputs: &[A]) -> Result<&TransitionState<'a, S>, &'static str> {
        for input in inputs {
            self.step(input)?;
        }

        Ok(&self.current)
    }
}