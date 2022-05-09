use std::hash::Hash;

use crate::dfa::delta::{TransitionState, δ};
use crate::model::state::State;

pub(crate) const ERR_INVALID_INPUT: &str = "Invalid input encountered";

const EXPECTED_TRANSITION_DEFINED: &str = "DFA expects all transition to states defined in transitions table";

#[allow(clippy::upper_case_acronyms)]
pub struct DFA<'a, A: Eq, S: Eq + Hash> {
    delta: δ<'a, A, S>,
    current: TransitionState<'a, S>,
}

impl<'a, A: Eq + Hash, S: Eq + Hash> DFA<'a, A, S>
{
    pub fn new(transitions: δ<'a, A, S>) -> Self {
        let current = transitions.get_initial_state();

        Self {
            current,
            delta: transitions,
        }
    }

    pub fn matches(&self) -> bool {
        matches!(self.current, State::Final(_))
    }

    pub fn reset(&mut self) {
        self.current = self.delta.get_initial_state();
    }

    pub fn step(&mut self, input: &A) -> Result<TransitionState<'a, S>, &'static str> {
        let transitions = self.delta.as_ref().get(self.current)
            .expect(EXPECTED_TRANSITION_DEFINED);

        self.current = transitions.get(input).ok_or(ERR_INVALID_INPUT)?;

        Ok(self.current)
    }

    pub fn steps(&mut self, inputs: &[A]) -> Result<TransitionState<'a, S>, &'static str> {
        for input in inputs {
            self.step(input)?;
        }

        Ok(self.current)
    }
}
