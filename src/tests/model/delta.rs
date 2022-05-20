#![allow(non_snake_case)]

use crate::model::delta::{
    ERR_DUPLICATE_DELTA_STATES,
    ERR_DUPLICATE_INPUT_TRANSITIONS,
    ERR_UNDEFINED_STATE_TRANSITION,
};
use crate::model::δ;
use crate::tests::assert_err;
use crate::tests::VALID_DELTA;

#[test]
fn given_a_collection_of_transitions_with_duplicate_input_transitions_we_should_get_an_err() {
    let δ = δ::new(vec![
        ('A', vec![(0, 'A'), (1, 'C')]),
        ('C', vec![(0, 'C'), (0, 'C'), (1, 'B')]), // (C) 0 -> C is defined twice
        ('B', vec![(0, 'C'), (1, 'B')]),
    ]);

    assert_err(ERR_DUPLICATE_INPUT_TRANSITIONS, &δ);
}

#[test]
fn given_a_collection_of_transitions_with_duplicate_states_we_should_get_an_err() {
    let δ = δ::new(vec![
        ('A', vec![(0, 'A'), (1, 'C')]),
        ('C', vec![(0, 'C'), (1, 'A')]), // \___ (C) is defined twice
        ('C', vec![(0, 'C'), (1, 'B')]), // /
    ]);

    assert_err(ERR_DUPLICATE_DELTA_STATES, &δ);
}

#[test]
fn given_a_collection_of_transitions_with_undefined_state_transitions_we_should_get_an_err() {
    let δ = δ::new(vec![
        ('A', vec![(0, 'A'), (1, 'C')]),
        ('C', vec![(0, 'C'), (1, 'D')]), // (D) is not a defined, only (A), (C) & (B) are defined
        ('B', vec![(0, 'C'), (1, 'B')]),
    ]);

    assert_err(ERR_UNDEFINED_STATE_TRANSITION, &δ);
}

#[test]
fn given_a_collection_of_valid_transitions_of_custom_type_should_et_you_transition_functions_δ() {
    use crate::tests::Sym::{S0, S1};
    use crate::tests::Sta::{SA, SB, SC};

    δ::new(vec![
        (SA, vec![(S0, SA), (S1, SB)]),
        (SC, vec![(S0, SC), (S1, SA)]),
        (SB, vec![(S0, SC), (S1, SB)]),
    ]).expect(VALID_DELTA);
}

#[test]
fn given_a_collection_of_valid_transitions_should_get_you_transition_functions_δ() {
    δ::new(vec![
        ('A', vec![(0, 'A'), (1, 'C')]),
        ('C', vec![(0, 'C'), (1, 'B')]),
        ('B', vec![(0, 'B'), (1, 'B')]),
    ]).expect(VALID_DELTA);
}
