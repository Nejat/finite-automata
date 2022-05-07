use crate::deterministic::alphabet::Alphabet;
use crate::deterministic::fsm::FSM;
use crate::deterministic::state::{StateNode, States};
use crate::deterministic::transition::TransitionTable;

#[test]
fn given_a_valid_fsm_should_fail_incorrect_input() {
    let symbols = vec![0_u8, 1_u8];
    let alphabet = Alphabet::new(&symbols).expect("a valid alphabet");

    let states = vec![
        StateNode::Initial("A"),
        StateNode::Final("B"),
        StateNode::Interim("C"),
        StateNode::Interim("D"),
    ];

    let states = States::new(&states).expect("valid states");

    let transitions = vec![
        ("A", vec![(0, "A"), (1, "C")]),
        ("C", vec![(0, "A"), (1, "B")]),
        ("B", vec![(0, "B"), (1, "D")]),
        ("D", vec![(0, "A"), (1, "A")]),
    ];

    let transitions = TransitionTable::new(&states, &alphabet, transitions)
        .expect("a valid transitions table");

    let mut sut = FSM::new(transitions);
    let inputs = [0, 1, 1, 0, 1];

    sut.steps(&inputs);

    let actual = sut.matches();
    let expected = false;

    assert_eq!(expected, actual, "running failing input");
}

#[test]
fn given_a_valid_fsm_you_should_be_able_to_process_step_by_step() {
    let symbols = vec![0_u8, 1_u8];
    let alphabet = Alphabet::new(&symbols).expect("a valid alphabet");

    let states = vec![
        StateNode::Initial("A"),
        StateNode::Final("B"),
        StateNode::Interim("C"),
        StateNode::Interim("D"),
    ];

    let states = States::new(&states).expect("valid states");

    let transitions = vec![
        ("A", vec![(0, "A"), (1, "C")]),
        ("C", vec![(0, "A"), (1, "B")]),
        ("B", vec![(0, "B"), (1, "D")]),
        ("D", vec![(0, "A"), (1, "A")]),
    ];

    let transitions = TransitionTable::new(&states, &alphabet, transitions)
        .expect("a valid transitions table");

    let mut sut = FSM::new(transitions);
    let inputs = [0, 1, 1, 0, 0];

    for input in &inputs {
        sut.step(input);
    }

    let actual = sut.matches();
    let expected = true;

    assert_eq!(expected, actual);
}

#[test]
fn given_a_valid_fsm_you_should_be_able_to_process_steps() {
    let symbols = vec![0_u8, 1_u8];
    let alphabet = Alphabet::new(&symbols).expect("a valid alphabet");

    let states = vec![
        StateNode::Initial("A"),
        StateNode::Final("B"),
        StateNode::Interim("C"),
        StateNode::Interim("D"),
    ];

    let states = States::new(&states).expect("valid states");

    let transitions = vec![
        ("A", vec![(0, "A"), (1, "C")]),
        ("C", vec![(0, "A"), (1, "B")]),
        ("B", vec![(0, "B"), (1, "D")]),
        ("D", vec![(0, "A"), (1, "A")]),
    ];

    let transitions = TransitionTable::new(&states, &alphabet, transitions)
        .expect("a valid transitions table");

    let mut sut = FSM::new(transitions);
    let inputs = [0, 1, 1, 0];

    sut.steps(&inputs);

    let actual = sut.matches();
    let expected = true;

    assert_eq!(expected, actual);
}

#[test]
fn given_a_valid_fsm_you_should_be_able_to_reset_and_restart_steps() {
    let symbols = vec![0_u8, 1_u8];
    let alphabet = Alphabet::new(&symbols).expect("a valid alphabet");

    let states = vec![
        StateNode::Initial("A"),
        StateNode::Final("B"),
        StateNode::Interim("C"),
        StateNode::Interim("D"),
    ];

    let states = States::new(&states).expect("valid states");

    let transitions = vec![
        ("A", vec![(0, "A"), (1, "C")]),
        ("C", vec![(0, "A"), (1, "B")]),
        ("B", vec![(0, "B"), (1, "D")]),
        ("D", vec![(0, "A"), (1, "A")]),
    ];

    let transitions = TransitionTable::new(&states, &alphabet, transitions)
        .expect("a valid transitions table");

    let mut sut = FSM::new(transitions);
    let inputs = [0, 1, 1, 0];

    // same as below without reset, expected to fail
    sut.steps(&inputs[..=1]);
    sut.steps(&inputs[1..]);

    let actual = sut.matches();
    let expected = false;

    assert_eq!(expected, actual);

    sut.reset();

    sut.steps(&inputs[..=1]);

    sut.reset();

    sut.steps(&inputs[1..]);

    let actual = sut.matches();
    let expected = true;

    assert_eq!(expected, actual);
}
