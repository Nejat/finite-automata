use crate::dfa;
use crate::dfa::delta::{
    ERR_DANGLING_STATE,
    ERR_DUPED_INPUT_TRANSITION,
    ERR_DUPED_TRANSITION,
    ERR_INCOMPLETE_INPUT_TRANSITIONS,
    ERR_MISSING_FINAL_STATE_TRANSITION,
    ERR_MISSING_INITIAL_STATE_TRANSITION,
    ERR_MISSING_STATE_TRANSITION,
    ERR_REDEFINED_INPUT_TRANSITION,
    ERR_UNDEFINED_SYMBOL,
    ERR_UNDEFINED_TRANSITION_STATE,
};
use crate::model::{Q, Σ};
use crate::tests::{VALID_DELTA, VALID_SIGMA, VALID_STATES};
use crate::tests::assert_err;

#[test]
fn given_a_collection_of_transitions_with_dangling_states_we_should_get_an_err() {
    let states = ['A', 'C', 'D', 'B'];

    let delta = vec![
        ('A', vec![(0, 'A'), (1, 'B')]),
        ('C', vec![(0, 'C'), (1, 'B')]),
        ('B', vec![(0, 'B'), (1, 'A')]),
    ];

    assert_delta_err(&states, delta, ERR_DANGLING_STATE);
}

#[test]
fn given_a_collection_of_transitions_with_dangling_initial_state_should_give_you_a_transition_table() {
    let symbols = [0, 1];
    let states = ['A', 'C', 'D', 'B'];
    let sigma = Σ::new(&symbols).expect(VALID_SIGMA);
    let mut q = Q::new(&states).expect(VALID_STATES);

    let delta = vec![
        ('A', vec![(0, 'A'), (1, 'C')]),
        ('C', vec![(0, 'C'), (1, 'B')]),
        ('B', vec![(0, 'B'), (1, 'B')]),
    ];

    dfa::δ::new(&mut q, &sigma, delta, 'A', vec!['B']).expect(VALID_DELTA);
}

#[test]
fn given_a_collection_of_transitions_with_duplicate_input_transitions_we_should_get_an_err() {
    let states = ['A', 'C', 'D', 'B'];

    let delta = vec![
        ('A', vec![(0, 'A'), (1, 'C')]),
        ('C', vec![(0, 'C'), (0, 'C'), (1, 'B')]),
        ('B', vec![(0, 'C'), (1, 'B')]),
    ];

    assert_delta_err(&states, delta, ERR_DUPED_INPUT_TRANSITION);
}

#[test]
fn given_a_collection_of_transitions_with_duplicate_states_we_should_get_an_err() {
    let states = ['A', 'C', 'D', 'B'];

    let delta = vec![
        ('A', vec![(0, 'A'), (1, 'C')]),
        ('C', vec![(0, 'C'), (1, 'A')]),
        ('C', vec![(0, 'C'), (1, 'B')]),
    ];

    assert_delta_err(&states, delta, ERR_DUPED_TRANSITION);
}

#[test]
fn given_a_collection_of_transitions_with_missing_final_state_we_should_get_an_err() {
    let states = ['A', 'C', 'D', 'B'];

    let delta = vec![
        ('A', vec![(0, 'A'), (1, 'C')]),
        ('C', vec![(0, 'C'), (1, 'A')]),
    ];

    assert_delta_err(&states, delta, ERR_MISSING_FINAL_STATE_TRANSITION);
}

#[test]
fn given_a_collection_of_transitions_with_missing_initial_state_we_should_get_an_err() {
    let states = ['A', 'C', 'D', 'B'];

    let delta = vec![
        ('B', vec![(0, 'B'), (1, 'C')]),
        ('C', vec![(0, 'C'), (1, 'B')]),
    ];

    assert_delta_err(&states, delta, ERR_MISSING_INITIAL_STATE_TRANSITION);
}

#[test]
fn given_a_collection_of_transitions_with_missing_input_transitions_we_should_get_an_err() {
    let states = ['A', 'C', 'B'];

    let delta = vec![
        ('A', vec![(0, 'A'), (1, 'C')]),
        ('C', vec![(0, 'C'), (1, 'A')]),
        ('D', vec![(0, 'C'), (1, 'B')]),
    ];

    assert_delta_err(&states, delta, ERR_UNDEFINED_TRANSITION_STATE);
}

#[test]
fn given_a_collection_of_transitions_with_missing_state_transitions_we_should_get_an_err() {
    let states = ['A', 'C', 'D', 'B'];

    let delta = vec![
        ('A', vec![(0, 'A'), (1, 'C')]),
        ('B', vec![(0, 'C'), (1, 'B')]),
    ];

    assert_delta_err(&states, delta, ERR_MISSING_STATE_TRANSITION);
}

#[test]
fn given_a_collection_of_transitions_with_redefined_input_transitions_we_should_get_an_err() {
    let states = vec!['A', 'C', 'B'];

    let delta = vec![
        ('A', vec![(0, 'A'), (1, 'C')]),
        ('C', vec![(0, 'C'), (0, 'B'), (1, 'B')]),
        ('B', vec![(0, 'C'), (1, 'B')]),
    ];

    assert_delta_err(&states, delta, ERR_REDEFINED_INPUT_TRANSITION);
}

#[test]
fn given_a_collection_of_transitions_with_undefined_states_we_should_get_an_err() {
    let states = vec!['A', 'C', 'B'];

    let delta = vec![
        ('A', vec![(0, 'A'), (1, 'C')]),
        ('C', vec![(0, 'C')]),
        ('B', vec![(0, 'C'), (1, 'B')]),
    ];

    assert_delta_err(&states, delta, ERR_INCOMPLETE_INPUT_TRANSITIONS);
}

#[test]
fn given_a_collection_of_transitions_with_undefined_symbols_we_should_get_an_err() {
    let states = vec!['A', 'C', 'B'];

    let delta = vec![
        ('A', vec![(0, 'A'), (1, 'C'), (2, 'B')]),
        ('C', vec![(0, 'C')]),
        ('B', vec![(0, 'C'), (1, 'B')]),
    ];

    assert_delta_err(&states, delta, ERR_UNDEFINED_SYMBOL);
}

#[test]
fn given_a_collection_of_valid_state_transitions_should_give_you_a_transition_table() {
    use crate::tests::Sym::{S0, S1};
    use crate::tests::Sta::{SA, SB, SC};

    let symbols = [S0, S1];
    let states = [SA, SB, SC];

    let sigma = Σ::new(&symbols).expect(VALID_SIGMA);
    let mut q = Q::new(&states).expect(VALID_STATES);

    let delta = vec![
        (SA, vec![(S0, SA), (S1, SB)]),
        (SC, vec![(S0, SC), (S1, SA)]),
        (SB, vec![(S0, SC), (S1, SB)]),
    ];

    dfa::δ::new(&mut q, &sigma, delta, SA, vec![SB]).expect(VALID_DELTA);
}

fn assert_delta_err(states: &[char], case: Vec<(char, Vec<(u8, char)>)>, expected: &str) {
    let symbols = [0, 1];
    let sigma = Σ::new(&symbols).expect(VALID_SIGMA);
    let mut q = Q::new(states).expect(VALID_STATES);

    let sut = dfa::δ::new(&mut q, &sigma, case, 'A', vec!['B']);

    assert_err(expected, &sut);
}
