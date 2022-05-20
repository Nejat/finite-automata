use crate::automata::{ERR_INVALID_INPUT, NFA};
use crate::model::{F, Q, δ, Σ};
use crate::tests::{
    STEPS_NO_ERRORS, VALID_DELTA, VALID_FINAL_STATES,
    VALID_SIGMA, VALID_STATES,
};
use crate::tests::assert_err;
use crate::tests::automata::nfa::assert_steps_case;

const VALID_NFA: &str = "a valid nfa";

#[test]
fn given_a_valid_nfa_should_fail_incorrect_input() {
    let expected = false;

    let δ = vec![
        ('A', vec![(0, 'A'), (1, 'C')]),
        ('C', vec![(0, 'A'), (1, 'B')]),
        ('B', vec![(0, 'B'), (1, 'D')]),
        ('D', vec![(0, 'A'), (1, 'A')]),
    ];

    assert_steps_case(
        vec!['A', 'B', 'C', 'D'], δ, expected,
        |sut| {
            let inputs = [0, 1, 1, 0, 1];

            sut.steps(&inputs).expect(STEPS_NO_ERRORS);
        },
    );
}

#[test]
fn given_a_valid_nfa_should_fail_invalid_input() {
    let states = vec!['A', 'B', 'C', 'D'];
    let Σ = Σ::new(vec![0, 1]).expect(VALID_SIGMA);
    let q = Q::new(states).expect(VALID_STATES);

    let δ = vec![
        ('A', vec![(0, 'A'), (1, 'C')]),
        ('C', vec![(0, 'A'), (1, 'B')]),
        ('B', vec![(0, 'B'), (1, 'D')]),
        ('D', vec![(0, 'A'), (1, 'A')]),
    ];

    let δ = δ::new(δ).expect(VALID_DELTA);
    let final_states = F::new(vec!['B']).expect(VALID_FINAL_STATES);
    let mut sut = NFA::new(q, Σ, δ, 'A', &final_states).expect(VALID_NFA);

    let inputs = [0, 1, 1, 2];
    let actual = sut.steps(&inputs);

    assert_err(ERR_INVALID_INPUT, &actual);
}

#[test]
fn given_a_valid_nfa_you_should_be_able_to_process_step_by_step() {
    let expected = true;

    let δ = vec![
        ('A', vec![(0, 'A'), (1, 'C')]),
        ('C', vec![(0, 'A'), (1, 'B')]),
        ('B', vec![(0, 'B'), (1, 'D')]),
        ('D', vec![(0, 'A'), (1, 'A')]),
    ];

    assert_steps_case(
        vec!['A', 'B', 'C', 'D'], δ, expected, |sut| {
            let inputs = [0, 1, 1, 0, 0];

            for input in &inputs {
                sut.step(input).expect(STEPS_NO_ERRORS);
            }
        },
    );
}

#[test]
fn given_a_valid_nfa_you_should_be_able_to_process_steps() {
    let expected = true;

    let δ = vec![
        ('A', vec![(0, 'A'), (1, 'C')]),
        ('C', vec![(0, 'A'), (1, 'B')]),
        ('B', vec![(0, 'B'), (1, 'D')]),
        ('D', vec![(0, 'A'), (1, 'A')]),
    ];

    assert_steps_case(
        vec!['A', 'B', 'C', 'D'], δ, expected,
        |sut| {
            let inputs = [0, 1, 1, 0];

            sut.steps(&inputs).expect(STEPS_NO_ERRORS);
        },
    );
}

#[test]
fn given_a_valid_nfa_you_should_be_able_to_reset_and_restart_steps() {
    let expected = true;

    let δ = vec![
        ('A', vec![(0, 'A'), (1, 'C')]),
        ('C', vec![(0, 'A'), (1, 'B')]),
        ('B', vec![(0, 'B'), (1, 'D')]),
        ('D', vec![(0, 'A'), (1, 'A')]),
    ];

    assert_steps_case(
        vec!['A', 'B', 'C', 'D'], δ, expected,
        |sut| {
            let inputs = [0, 1, 1, 1, 0];

            // same as below without reset, expected to fail
            sut.steps(&inputs[..2]).expect(STEPS_NO_ERRORS);
            sut.steps(&inputs[2..]).expect(STEPS_NO_ERRORS);

            let actual = sut.matches();
            let expected = false;

            assert_eq!(expected, actual);

            sut.reset();

            sut.steps(&inputs[..2]).expect(STEPS_NO_ERRORS);

            sut.reset();

            sut.steps(&inputs[2..]).expect(STEPS_NO_ERRORS);
        },
    );
}

