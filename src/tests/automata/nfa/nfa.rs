#![allow(non_snake_case)]

use crate::automata::{ERR_INVALID_INPUT, NFA};
use crate::model::{F, Q, δ, Σ};
use crate::tests::{
    assert_err, STEPS_NO_ERRORS, VALID_DELTA,
    VALID_FINAL_STATES, VALID_SIGMA, VALID_STATES,
};
use crate::tests::automata::nfa::{assert_steps_case, VALID_NFA};

#[test]
fn given_a_valid_nfa_should_fail_incorrect_input() {
    let expected = false;
    let states = vec!['A', 'B'];

    let δ = vec![
        ('A', vec![(0, 'A'), (1, 'A'), (0, 'B')]),
        ('B', vec![]),
    ];

    assert_steps_case(states, δ, expected, |sut| {
        let inputs = [0, 1, 1, 0, 1];

        sut.steps(&inputs).expect(STEPS_NO_ERRORS);
    });
}

#[test]
fn given_a_valid_nfa_should_fail_invalid_input() {
    let Σ = Σ::new(vec![0_u8, 1_u8]).expect(VALID_SIGMA);
    let Q = Q::new(vec!['A', 'B']).expect(VALID_STATES);
    let F = F::new(vec!['B']).expect(VALID_FINAL_STATES);

    let δ = vec![
        ('A', vec![(0, 'A'), (1, 'A'), (0, 'B')]),
        ('B', vec![]),
    ];

    let δ = δ::new(δ).expect(VALID_DELTA);

    let mut sut = NFA::new(Q, Σ, δ, 'A', &F).expect(VALID_NFA);
    let inputs = [0, 1, 1, 2];
    let actual = sut.steps(&inputs);

    assert_err(ERR_INVALID_INPUT, &actual);
}

#[test]
fn given_a_valid_nfa_you_should_be_able_to_process_step_by_step() {
    let expected = true;
    let states = vec!['A', 'B'];

    let δ = vec![
        ('A', vec![(0, 'A'), (1, 'A'), (0, 'B')]),
        ('B', vec![]),
    ];

    assert_steps_case(states, δ, expected, |sut| {
        let inputs = [0, 1, 0, 1, 1, 0];

        for input in &inputs {
            sut.step(input).expect(STEPS_NO_ERRORS);
        }
    });
}

#[test]
fn given_a_valid_nfa_you_should_be_able_to_process_steps() {
    let expected = true;
    let states = vec!['A', 'B'];

    let δ = vec![
        ('A', vec![(0, 'A'), (1, 'A'), (0, 'B')]),
        ('B', vec![]),
    ];

    assert_steps_case(states, δ, expected, |sut| {
        let inputs = [0, 1, 0, 1, 1, 0];

        sut.steps(&inputs).expect(STEPS_NO_ERRORS);
    });
}

#[test]
fn given_a_valid_nfa_you_should_be_able_to_reset_and_restart_steps() {
    let expected = true;
    let states = vec!['A', 'B', 'C'];

    let δ = vec![
        ('A', vec![(1, 'C')]),
        ('C', vec![(0, 'C'), (1, 'A'), (0, 'B')]),
        ('B', vec![]),
    ];

    assert_steps_case(states, δ, expected, |sut| {
        let inputs = [0, 0, 1, 0, 1, 1, 0];

        // same as below without reset, expected to fail
        sut.steps(&inputs[..2]).expect(STEPS_NO_ERRORS);
        sut.steps(&inputs[2..]).expect(STEPS_NO_ERRORS);

        let actual = sut.matches();
        let expected = false;

        assert_eq!(expected, actual, "Intermediate Step");

        sut.reset();

        sut.steps(&inputs[..2]).expect(STEPS_NO_ERRORS);

        sut.reset();

        sut.steps(&inputs[2..]).expect(STEPS_NO_ERRORS);
    });
}
