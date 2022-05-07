use std::hash::Hash;

use crate::deterministic::state::{State, StateNode};
use crate::deterministic::transition::TransitionTable;

pub(crate) const ERR_INVALID_INPUT: &str = "Invalid input encountered";

const EXPECTED_TRANSITION_DEFINED: &str = "FSM expects all transition to states defined in transitions table";

#[allow(clippy::upper_case_acronyms)]
pub struct FSM<'a, A: Eq, S: Eq + Hash> {
    transitions: TransitionTable<'a, A, S>,
    current: &'a StateNode<State<'a, S>>,
}

impl<'a, A: Eq + Hash, S: Eq + Hash> FSM<'a, A, S>
{
    pub fn new(transitions: TransitionTable<'a, A, S>) -> Self {
        let current = transitions.get_initial_state();

        Self {
            current,
            transitions,
        }
    }

    pub fn matches(&self) -> bool {
        matches!(self.current, StateNode::Final(_))
    }

    pub fn reset(&mut self) {
        self.current = self.transitions.get_initial_state();
    }

    pub fn step(&mut self, input: &A) ->  Result<&'a StateNode<State<'a, S>>, &'static str> {
        let transitions = self.transitions.as_ref().get(self.current)
            .expect(EXPECTED_TRANSITION_DEFINED);

        self.current = transitions.get(input).ok_or(ERR_INVALID_INPUT)?;

        Ok(self.current)
    }

    pub fn steps(&mut self, inputs: &[A]) ->  Result<&'a StateNode<State<'a, S>>, &'static str> {
        for input in inputs {
            self.step(input)?;
        }

        Ok(self.current)
    }
}