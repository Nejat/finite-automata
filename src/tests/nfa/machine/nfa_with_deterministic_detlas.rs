use crate::model::{Q, Σ};
use crate::nfa::{self, NFA};
use crate::nfa::machine::ERR_INVALID_INPUT;
use crate::tests::{STEPS_NO_ERRORS, VALID_DELTA, VALID_SIGMA, VALID_STATES};
use crate::tests::assert_err;
use crate::tests::nfa::machine::assert_steps_case;

#[test]
fn given_a_valid_fsm_should_fail_incorrect_input() {
    let expected = false;
    let states = ['A', 'B', 'C', 'D'];

    let delta = vec![
        ('A', vec![(0, 'A'), (1, 'C')]),
        ('C', vec![(0, 'A'), (1, 'B')]),
        ('B', vec![(0, 'B'), (1, 'D')]),
        ('D', vec![(0, 'A'), (1, 'A')]),
    ];

    assert_steps_case(&states, delta, expected, |sut| {
        let inputs = [0, 1, 1, 0, 1];

        sut.steps(&inputs).expect(STEPS_NO_ERRORS);
    });
}

#[test]
fn given_a_valid_fsm_should_fail_invalid_input() {
    let symbols = [0, 1];
    let states = ['A', 'B', 'C', 'D'];
    let sigma = Σ::new(&symbols).expect(VALID_SIGMA);
    let mut q = Q::new(&states).expect(VALID_STATES);

    let delta = vec![
        ('A', vec![(0, 'A'), (1, 'C')]),
        ('C', vec![(0, 'A'), (1, 'B')]),
        ('B', vec![(0, 'B'), (1, 'D')]),
        ('D', vec![(0, 'A'), (1, 'A')]),
    ];

    let delta = nfa::δ::new(&mut q, &sigma, delta, 'A', vec!['B']).expect(VALID_DELTA);
    let mut sut = NFA::new(delta);

    let inputs = [0, 1, 1, 2];
    let actual = sut.steps(&inputs);

    assert_err(ERR_INVALID_INPUT, &actual);
}

#[test]
fn given_a_valid_fsm_you_should_be_able_to_process_step_by_step() {
    let expected = true;
    let states = ['A', 'B', 'C', 'D'];

    let delta = vec![
        ('A', vec![(0, 'A'), (1, 'C')]),
        ('C', vec![(0, 'A'), (1, 'B')]),
        ('B', vec![(0, 'B'), (1, 'D')]),
        ('D', vec![(0, 'A'), (1, 'A')]),
    ];

    assert_steps_case(&states, delta, expected, |sut| {
        let inputs = [0, 1, 1, 0, 0];

        for input in &inputs {
            sut.step(input).expect(STEPS_NO_ERRORS);
        }
    });
}

#[test]
fn given_a_valid_fsm_you_should_be_able_to_process_steps() {
    let expected = true;
    let states = ['A', 'B', 'C', 'D'];

    let delta = vec![
        ('A', vec![(0, 'A'), (1, 'C')]),
        ('C', vec![(0, 'A'), (1, 'B')]),
        ('B', vec![(0, 'B'), (1, 'D')]),
        ('D', vec![(0, 'A'), (1, 'A')]),
    ];

    assert_steps_case(&states, delta, expected, |sut| {
        let inputs = [0, 1, 1, 0];

        sut.steps(&inputs).expect(STEPS_NO_ERRORS);
    });
}

#[test]
fn given_a_valid_fsm_you_should_be_able_to_reset_and_restart_steps() {
    let expected = true;
    let states = ['A', 'B', 'C', 'D'];

    let delta = vec![
        ('A', vec![(0, 'A'), (1, 'C')]),
        ('C', vec![(0, 'A'), (1, 'B')]),
        ('B', vec![(0, 'B'), (1, 'D')]),
        ('D', vec![(0, 'A'), (1, 'A')]),
    ];

    assert_steps_case(&states, delta, expected, |sut| {
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
    });
}
