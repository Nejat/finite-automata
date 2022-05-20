#![allow(non_snake_case)]

use crate::automata::{
    DFA, ERR_DANGLING_STATE, ERR_UNDEFINED_FINAL_STATE,
    ERR_UNDEFINED_INITIAL_STATE, ERR_UNREFERENCED_STATE_Q,
};
use crate::automata::dfa::{
    ERR_DUPLICATE_INPUT_TRANSITION,
    ERR_INCOMPLETE_INPUT_TRANSITIONS,
    ERR_UNDEFINED_SYMBOL,
};
use crate::model::{F, Q, δ, Σ};
use crate::tests::{VALID_DELTA, VALID_FINAL_STATES, VALID_SIGMA, VALID_STATES};
use crate::tests::assert_err;
use crate::tests::automata::VALID_DFA;

#[test]
fn given_a_collection_of_transitions_with_dangling_states_we_should_get_an_err() {
    let states = vec!['A', 'C', 'B'];

    let δ = vec![
        ('A', vec![(0, 'A'), (1, 'B')]),
        ('C', vec![(0, 'C'), (1, 'B')]), // no transitions to (C)
        ('B', vec![(0, 'B'), (1, 'A')]),
    ];

    assert_dfa_configuration(states, δ, ERR_DANGLING_STATE);
}

#[test]
fn given_a_collection_of_transitions_with_no_transitions_to_initial_state_should_give_you_a_transition_table() {
    let Σ = Σ::new(vec![0, 1]).expect(VALID_SIGMA);
    let Q = Q::new(vec!['A', 'C', 'B']).expect(VALID_STATES);
    let final_states = F::new(vec!['B']).expect(VALID_FINAL_STATES);

    let δ = δ::new(vec![
        ('A', vec![(0, 'A'), (1, 'C')]),
        ('C', vec![(0, 'C'), (1, 'B')]),
        ('B', vec![(0, 'B'), (1, 'B')]),
    ]).expect(VALID_DELTA);

    DFA::new(Q, &Σ, δ, 'A', &final_states).expect(VALID_DFA);
}

#[test]
fn given_a_collection_of_transitions_with_missing_final_state_we_should_get_an_err() {
    let states = vec!['A', 'C'];

    // Final state ((B)), configured in assert_dfa_configuration, is missing here
    let δ = vec![
        ('A', vec![(0, 'A'), (1, 'C')]),
        ('C', vec![(0, 'C'), (1, 'A')]),
    ];

    assert_dfa_configuration(states, δ, ERR_UNDEFINED_FINAL_STATE);
}

#[test]
fn given_a_collection_of_transitions_with_missing_initial_state_we_should_get_an_err() {
    let states = vec!['C', 'B'];

    // Initial state >(A), configured in assert_delta_configuration, is missing here
    let δ = vec![
        ('B', vec![(0, 'B'), (1, 'C')]),
        ('C', vec![(0, 'C'), (1, 'B')]),
    ];

    assert_dfa_configuration(states, δ, ERR_UNDEFINED_INITIAL_STATE);
}

#[test]
fn given_a_collection_of_transitions_with_duplicate_input_transitions_we_should_get_an_err() {
    let states = vec!['A', 'C', 'B'];

    let δ = vec![
        ('A', vec![(0, 'A'), (1, 'C')]),
        ('C', vec![(0, 'C'), (0, 'B'), (1, 'B')]), // input symbol '0' is duplicated here
        ('B', vec![(0, 'C'), (1, 'B')]),
    ];

    assert_dfa_configuration(states, δ, ERR_DUPLICATE_INPUT_TRANSITION);
}

#[test]
fn given_a_collection_of_transitions_with_incomplete_input_transitions_we_should_get_an_err() {
    let states = vec!['A', 'C', 'B'];

    let δ = vec![
        ('A', vec![(0, 'A'), (1, 'C')]),
        ('C', vec![(0, 'C')]),              // input symbol '1' is not defined here
        ('B', vec![(0, 'C'), (1, 'B')]),
    ];

    assert_dfa_configuration(states, δ, ERR_INCOMPLETE_INPUT_TRANSITIONS);
}

#[test]
fn given_a_collection_of_transitions_with_undefined_symbols_we_should_get_an_err() {
    let states = vec!['A', 'C', 'B'];

    let δ = vec![
        ('A', vec![(0, 'A'), (1, 'C'), (2, 'B')]), // input symbol '2' is not defined in assert_dfa_configuration
        ('C', vec![(0, 'C'), (1, 'A')]),
        ('B', vec![(0, 'C'), (1, 'B')]),
    ];

    assert_dfa_configuration(states, δ, ERR_UNDEFINED_SYMBOL);
}

#[test]
fn given_a_missing_state_transition_should_get_an_err() {
    use crate::tests::Sym::{S0, S1};
    use crate::tests::Sta::{SA, SB, SC, SD};

    let Σ = Σ::new(vec![S0, S1]).expect(VALID_SIGMA);
    let Q = Q::new(vec![SA, SB, SC, SD]).expect(VALID_STATES);
    let final_states = F::new(vec![SB]).expect(VALID_FINAL_STATES);

    let δ = δ::new(vec![
        (SA, vec![(S0, SA), (S1, SB)]),
        (SC, vec![(S0, SC), (S1, SA)]),
        (SB, vec![(S0, SC), (S1, SB)]),
    ]).expect(VALID_DELTA);

    let sut = DFA::new(Q, &Σ, δ, SA, &final_states);

    assert_err(ERR_UNREFERENCED_STATE_Q, &sut);
}

#[test]
fn given_a_collection_of_valid_state_transitions_should_give_you_a_transition_table() {
    use crate::tests::Sym::{S0, S1};
    use crate::tests::Sta::{SA, SB, SC};

    let Σ = Σ::new(vec![S0, S1]).expect(VALID_SIGMA);
    let Q = Q::new(vec![SA, SB, SC]).expect(VALID_STATES);
    let final_states = F::new(vec![SB]).expect(VALID_FINAL_STATES);

    let δ = δ::new(vec![
        (SA, vec![(S0, SA), (S1, SB)]),
        (SC, vec![(S0, SC), (S1, SA)]),
        (SB, vec![(S0, SC), (S1, SB)]),
    ]).expect(VALID_DELTA);

    DFA::new(Q, &Σ, δ, SA, &final_states).expect(VALID_DFA);
}

fn assert_dfa_configuration(tags: Vec<char>, case: Vec<(char, Vec<(u8, char)>)>, expected: &str) {
    let Σ = Σ::new(vec![0, 1]).expect(VALID_SIGMA);
    let Q = Q::new(tags).expect(VALID_STATES);
    let δ = δ::new(case).expect(VALID_DELTA);
    let final_states = F::new(vec!['B']).expect(VALID_FINAL_STATES);

    let sut = DFA::new(Q, &Σ, δ, 'A', &final_states);

    assert_err(expected, &sut);
}
