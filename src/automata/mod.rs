//!

use std::collections::{HashMap, HashSet};
use std::hash::Hash;

pub use dfa::DFA;
pub use nfa::NFA;

use crate::model::{F, δ};
use crate::model::state::{Phase, Q, State};
use crate::UNREACHABLE_ERR;

pub(crate) mod dfa;
pub(crate) mod nfa;

pub(crate) const ERR_DANGLING_STATE: &str = "Transition functions has a dangling state";
pub(crate) const ERR_INVALID_INPUT: &str = "Undefined Input Symbol";
pub(crate) const ERR_UNDEFINED_FINAL_STATE: &str = "Final state f is not defined in transition functions δ";
pub(crate) const ERR_UNDEFINED_INITIAL_STATE: &str = "Initial state q0 is not defined in transition functions δ";
pub(crate) const ERR_UNREFERENCED_STATE_Q: &str = "Q contains a state that does not have a transition function defined";

type Transitions<A, S> = HashMap<State<S>, HashMap<A, Vec<S>>>;

#[allow(non_snake_case)]
fn convert_to_transitions<A: Eq, S: Eq + Hash>(
    Q: Q<S>, q0: S, F: &F<S>, δ: &δ<A, S>,
) -> Result<Transitions<A, S>, &'static str> {
    let states: Vec<_> = Q.into();
    let δ = δ.states_transitioned().collect::<HashSet<_>>();
    let q0 = vec![q0];

    let states = states.into_iter()
        .filter(|state| *state == q0[0] || δ.contains(state))
        .map(|tag| {
            let tags = vec![tag];
            let initial = tags == q0;
            let r#final = F.contains(&tags);

            let phase = match (initial, r#final) {
                (false, false) => Phase::Interim,
                (true, false) => Phase::Initial,
                (false, true) => Phase::Final,
                (true, true) => Phase::Both,
            };

            Ok((State::new(tags, phase)?, <HashMap<A, Vec<S>>>::new()))
        })
        .collect::<Result<HashMap<_, _>, _>>()?;

    Ok(states)
}

fn get_initial_state<A, S: Eq>(transitions: &Transitions<A, S>) -> State<S> {
    transitions.keys().find(|key| key.is_initial()).expect(UNREACHABLE_ERR).clone()
}

#[allow(non_snake_case)]
fn validate_fa_configuration<A, S: Eq + Hash>(
    Q: &Q<S>, δ: &δ<A, S>, q0: &S, F: &F<S>,
) -> Result<(), &'static str> {
    if Q.iter().any(|q| δ.iter().all(|(state, _)| state != q)) {
        Err(ERR_UNREFERENCED_STATE_Q)
    } else if δ.iter().all(|(state, _)| state != q0) {
        Err(ERR_UNDEFINED_INITIAL_STATE)
    } else if δ.iter().all(|(state, _)| F.iter().all(|f| !f.contains(state))) {
        Err(ERR_UNDEFINED_FINAL_STATE)
    } else {
        Ok(())
    }
}

