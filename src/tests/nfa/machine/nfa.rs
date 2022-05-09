use crate::model::{Q, State, Σ};
use crate::nfa::{self, NFA};
use crate::nfa::machine::ERR_INVALID_INPUT;

#[test]
fn given_a_valid_fsm_should_fail_incorrect_input() {
    let symbols = vec![0_u8, 1_u8];
    let sigma = Σ::new(&symbols).expect("valid sigma");

    let states = vec![
        State::Initial("A"),
        State::Final("B"),
    ];

    let q = Q::new(&states).expect("valid states");

    let delta = vec![
        ("A", vec![(0, "A"), (1, "A"), (0, "B")]),
        ("B", vec![]),
    ];

    let delta = nfa::δ::new(&q, &sigma, delta).expect("a valid delta table");

    let mut sut = NFA::new(delta);
    let inputs = [0, 1, 1, 0, 1];

    sut.steps(&inputs).expect("expect no errors");

    let actual = sut.matches();
    let expected = false;

    assert_eq!(expected, actual);
}

#[test]
fn given_a_valid_fsm_should_fail_invalid_input() {
    let symbols = vec![0_u8, 1_u8];
    let sigma = Σ::new(&symbols).expect("valid sigma");

    let states = vec![
        State::Initial("A"),
        State::Final("B"),
        State::Interim("C"),
    ];

    let q = Q::new(&states).expect("valid states");

    let delta = vec![
        ("A", vec![(0, "A"), (1, "A"), (0, "B")]),
        ("B", vec![]),
    ];

    let delta = nfa::δ::new(&q, &sigma, delta).expect("a valid delta table");

    let mut sut = NFA::new(delta);
    let inputs = [0, 1, 1, 2];
    let actual = sut.steps(&inputs);

    match actual {
        Ok(_) => panic!("Expected Err: {ERR_INVALID_INPUT}"),
        Err(err) => assert_eq!(ERR_INVALID_INPUT, err)
    }
}

#[test]
fn given_a_valid_fsm_you_should_be_able_to_process_step_by_step() {
    let symbols = vec![0_u8, 1_u8];
    let sigma = Σ::new(&symbols).expect("valid sigma");

    let states = vec![
        State::Initial("A"),
        State::Final("B"),
        State::Interim("C"),
    ];

    let q = Q::new(&states).expect("valid states");

    let delta = vec![
        ("A", vec![(0, "A"), (1, "A"), (0, "B")]),
        ("B", vec![]),
    ];

    let delta = nfa::δ::new(&q, &sigma, delta).expect("a valid delta table");

    let mut sut = NFA::new(delta);
    let inputs = [0, 1, 0, 1, 1, 0];

    for input in &inputs {
        sut.step(input).expect("expect no errors");
    }

    let actual = sut.matches();
    let expected = true;

    assert_eq!(expected, actual);
}

#[test]
fn given_a_valid_fsm_you_should_be_able_to_process_steps() {
    let symbols = vec![0_u8, 1_u8];
    let sigma = Σ::new(&symbols).expect("valid sigma");

    let states = vec![
        State::Initial("A"),
        State::Final("B"),
        State::Interim("C"),
    ];

    let q = Q::new(&states).expect("valid states");

    let delta = vec![
        ("A", vec![(0, "A"), (1, "A"), (0, "B")]),
        ("B", vec![]),
    ];

    let delta = nfa::δ::new(&q, &sigma, delta).expect("a valid delta table");

    let mut sut = NFA::new(delta);
    let inputs = [0, 1, 0, 1, 1, 0];

    sut.steps(&inputs).expect("expect no errors");

    let actual = sut.matches();
    let expected = true;

    assert_eq!(expected, actual);
}

#[test]
fn given_a_valid_fsm_you_should_be_able_to_reset_and_restart_steps() {
    let symbols = vec![0_u8, 1_u8];
    let sigma = Σ::new(&symbols).expect("valid sigma");

    let states = vec![
        State::Initial("A"),
        State::Final("B"),
        State::Interim("C"),
    ];

    let q = Q::new(&states).expect("valid states");

    let delta = vec![
        ("A", vec![(1, "C")]),
        ("C", vec![(0, "C"), (1, "C"), (0, "B")]),
        ("B", vec![]),
    ];

    let delta = nfa::δ::new(&q, &sigma, delta).expect("a valid delta table");

    let mut sut = NFA::new(delta);
    let inputs = [0, 0, 1, 0, 1, 1, 0];

    // same as below without reset, expected to fail
    sut.steps(&inputs[..2]).expect("expect no errors");
    sut.steps(&inputs[2..]).expect("expect no errors");

    let actual = sut.matches();
    let expected = false;

    assert_eq!(expected, actual);

    sut.reset();

    sut.steps(&inputs[..2]).expect("expect no errors");

    sut.reset();

    sut.steps(&inputs[2..]).expect("expect no errors");

    let actual = sut.matches();
    let expected = true;

    assert_eq!(expected, actual);
}
