use crate::model::{Q, Σ};
use crate::nfa;
use crate::nfa::delta::{
    ERR_DANGLING_STATE,
    ERR_DUPED_INPUT_TRANSITION,
    ERR_DUPED_TRANSITION,
    ERR_MISSING_FINAL_STATE_TRANSITION,
    ERR_MISSING_INITIAL_STATE_TRANSITION,
    ERR_MISSING_STATE_TRANSITION,
    ERR_UNDEFINED_SYMBOL,
    ERR_UNDEFINED_TRANSITION_STATE,
};
use crate::tests::{VALID_DELTA, VALID_SIGMA, VALID_STATES};
use crate::tests::assert_err;

#[test]
fn given_a_collection_of_transitions_with_dangling_states_we_should_get_an_err() {
    let delta = vec![
        ('A', vec![(0, 'A'), (0, 'B')]),
        ('C', vec![(0, 'C'), (1, 'B')]),
        ('B', vec![(0, 'A'), (1, 'B')]),
    ];

    assert_delta_err(delta, ERR_DANGLING_STATE);
}

#[test]
fn given_a_collection_of_transitions_with_dangling_initial_state_should_give_you_a_transition_table() {
    let states = ['A', 'C', 'B'];
    let sigma = Σ::new(vec![0, 1]).expect("valid sigma");
    let mut q = Q::new(&states).expect("valid states");

    let delta = vec![
        ('A', vec![(0, 'A'), (1, 'C')]),
        ('C', vec![(0, 'C'), (1, 'B')]),
        ('B', vec![(0, 'B'), (1, 'B')]),
    ];

    nfa::δ::new(&mut q, sigma, delta, 'A', vec!['B']).expect(VALID_DELTA);
}

#[test]
fn given_a_collection_of_transitions_with_duplicate_input_transitions_we_should_get_an_err() {
    let delta = vec![
        ('A', vec![(0, 'A'), (1, 'C')]),
        ('C', vec![(0, 'C'), (0, 'C')]),
        ('B', vec![(0, 'C')]),
    ];

    assert_delta_err(delta, ERR_DUPED_INPUT_TRANSITION);
}

#[test]
fn given_a_collection_of_transitions_with_duplicate_states_we_should_get_an_err() {
    let delta = vec![
        ('A', vec![(0, 'A'), (1, 'C')]),
        ('C', vec![]),
        ('C', vec![(0, 'C'), (1, 'B')]),
    ];

    assert_delta_err(delta, ERR_DUPED_TRANSITION);
}

#[test]
fn given_a_collection_of_transitions_with_missing_final_state_we_should_get_an_err() {
    let delta = vec![
        ('A', vec![(0, 'A'), (1, 'C')]),
        ('C', vec![(1, 'A')]),
    ];

    assert_delta_err(delta, ERR_MISSING_FINAL_STATE_TRANSITION);
}

#[test]
fn given_a_collection_of_transitions_with_missing_initial_state_we_should_get_an_err() {
    let delta = vec![
        ('B', vec![(1, 'C')]),
        ('C', vec![(0, 'C'), (1, 'B')]),
    ];

    assert_delta_err(delta, ERR_MISSING_INITIAL_STATE_TRANSITION);
}

#[test]
fn given_a_collection_of_transitions_with_missing_input_transitions_we_should_get_an_err() {
    let delta = vec![
        ('A', vec![(0, 'A'), (1, 'C')]),
        ('C', vec![(0, 'C'), (1, 'A'), (1, 'B')]),
        ('D', vec![(0, 'C'), (1, 'B')]),
    ];

    assert_delta_err(delta, ERR_UNDEFINED_TRANSITION_STATE);
}

#[test]
fn given_a_collection_of_transitions_with_missing_state_transitions_we_should_get_an_err() {
    let delta = vec![
        ('A', vec![(0, 'A'), (1, 'C')]),
        ('B', vec![]),
    ];

    assert_delta_err(delta, ERR_MISSING_STATE_TRANSITION);
}

#[test]
fn given_a_collection_of_transitions_with_undefined_symbols_we_should_get_an_err() {
    let delta = vec![
        ('A', vec![(0, 'A'), (1, 'C'), (2, 'B')]),
        ('C', vec![(0, 'C')]),
        ('B', vec![(0, 'C'), (1, 'B')]),
    ];

    assert_delta_err(delta, ERR_UNDEFINED_SYMBOL);
}

#[test]
fn given_a_collection_of_valid_state_transitions_should_give_you_a_transition_table() {
    use crate::tests::Sym::{S0, S1};
    use crate::tests::Sta::{SA, SB, SC};

    let states = vec![SA, SB, SC];
    let sigma = Σ::new(vec![S0, S1]).expect("valid sigma");
    let mut q = Q::new(&states).expect("valid states");

    let delta = vec![
        (SA, vec![(S0, SC)]),
        (SC, vec![(S0, SB), (S0, SA)]),
        (SB, vec![]),
    ];

    nfa::δ::new(&mut q, sigma, delta, SA, vec![SB]).expect(VALID_DELTA);
}

fn assert_delta_err(case: Vec<(char, Vec<(u8, char)>)>, expected: &str) {
    let states = ['A', 'C', 'B'];
    let sigma = Σ::new(vec![0, 1]).expect(VALID_SIGMA);
    let mut q = Q::new(&states).expect(VALID_STATES);

    let sut = nfa::δ::new(&mut q, sigma, case, 'A', vec!['B']);

    assert_err(expected, &sut);
}
