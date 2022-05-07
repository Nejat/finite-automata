use std::hash::Hash;

use crate::deterministic::state::{State, StateNode};
use crate::deterministic::transition::TransitionTable;

const EXPECTED_INPUT_DEFINED: &str = "FSM expects all transition inputs defined in transition";
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

    pub fn step(&mut self, input: &A) ->  &'a StateNode<State<'a, S>> {
        let transitions = self.transitions.as_ref().get(self.current).expect(EXPECTED_TRANSITION_DEFINED);

        self.current = transitions.get(input).expect(EXPECTED_INPUT_DEFINED);

        self.current
    }

    pub fn steps(&mut self, inputs: &[A]) ->  &'a StateNode<State<'a, S>> {
        for input in inputs {
            self.step(input);
        }

        self.current
    }
}