use crate::deterministic::alphabet::Alphabet;
use crate::deterministic::state::{StateNode, States};
use crate::deterministic::transition::{
    ERR_DUPED_INPUT_TRANSITION,
    ERR_DUPED_TRANSITION,
    ERR_INCOMPLETE_INPUT_TRANSITIONS,
    ERR_MISSING_STATE_TRANSITION,
    ERR_REDEFINED_INPUT_TRANSITION,
    ERR_UNDEFINED_TRANSITION_STATE,
    TransitionTable
};

#[test]
fn given_a_collection_of_transitions_with_duplicate_input_transitions_should_get_an_err() {
    let symbols = vec![0, 1];
    let states = vec![
        StateNode::Initial("A"),
        StateNode::Interim("C"),
        StateNode::Final("B"),
    ];

    let alphabet = Alphabet::new(&symbols).expect("a valid alphabet");
    let states = States::new(&states).expect("valid states");

    let transitions = vec![
        ("A", vec![(0, "A"), (1, "C")]),
        ("C", vec![(0, "C"), (0, "C"), (1, "B")]),
        ("B", vec![(0, "C"), (1, "B")]),
    ];

    let sut = TransitionTable::new(&states, &alphabet, transitions);

    match sut {
        Ok(_) => panic!("Expected Err: {ERR_DUPED_INPUT_TRANSITION}"),
        Err(err) => assert_eq!(ERR_DUPED_INPUT_TRANSITION, err)
    }
}

#[test]
fn given_a_collection_of_transitions_with_duplicate_states_should_get_an_err() {
    let symbols = vec![0, 1];
    let states = vec![
        StateNode::Initial("A"),
        StateNode::Interim("C"),
        StateNode::Final("B"),
    ];

    let alphabet = Alphabet::new(&symbols).expect("a valid alphabet");
    let states = States::new(&states).expect("valid states");

    let transitions = vec![
        ("A", vec![(0, "A"), (1, "C")]),
        ("C", vec![(0, "C"), (1, "A")]),
        ("C", vec![(0, "C"), (1, "B")]),
    ];

    let sut = TransitionTable::new(&states, &alphabet, transitions);

    match sut {
        Ok(_) => panic!("Expected Err: {ERR_DUPED_TRANSITION}"),
        Err(err) => assert_eq!(ERR_DUPED_TRANSITION, err)
    }
}

#[test]
fn given_a_collection_of_transitions_with_missing_input_transitions_should_get_an_err() {
    let symbols = vec![0, 1];
    let states = vec![
        StateNode::Initial("A"),
        StateNode::Interim("C"),
        StateNode::Final("B"),
    ];

    let alphabet = Alphabet::new(&symbols).expect("a valid alphabet");
    let states = States::new(&states).expect("valid states");

    let transitions = vec![
        ("A", vec![(0, "A"), (1, "C")]),
        ("C", vec![(0, "C"), (1, "A")]),
        ("D", vec![(0, "C"), (1, "B")]),
    ];

    let sut = TransitionTable::new(&states, &alphabet, transitions);

    match sut {
        Ok(_) => panic!("Expected Err: {ERR_UNDEFINED_TRANSITION_STATE}"),
        Err(err) => assert_eq!(ERR_UNDEFINED_TRANSITION_STATE, err)
    }
}

#[test]
fn given_a_collection_of_transitions_with_missing_state_transitions_should_get_an_err() {
    let symbols = vec![0, 1];
    let states = vec![
        StateNode::Initial("A"),
        StateNode::Interim("C"),
        StateNode::Final("B"),
    ];

    let alphabet = Alphabet::new(&symbols).expect("a valid alphabet");
    let states = States::new(&states).expect("valid states");

    let transitions = vec![
        ("A", vec![(0, "A"), (1, "C")]),
        ("C", vec![(0, "C"), (1, "B")]),
    ];

    let sut = TransitionTable::new(&states, &alphabet, transitions);

    match sut {
        Ok(_) => panic!("Expected Err: {ERR_MISSING_STATE_TRANSITION}"),
        Err(err) => assert_eq!(ERR_MISSING_STATE_TRANSITION, err)
    }
}

#[test]
fn given_a_collection_of_transitions_with_redefined_input_transitions_should_get_an_err() {
    let symbols = vec![0, 1];
    let states = vec![
        StateNode::Initial("A"),
        StateNode::Interim("C"),
        StateNode::Final("B"),
    ];

    let alphabet = Alphabet::new(&symbols).expect("a valid alphabet");
    let states = States::new(&states).expect("valid states");

    let transitions = vec![
        ("A", vec![(0, "A"), (1, "C")]),
        ("C", vec![(0, "C"), (0, "B"), (1, "B")]),
        ("B", vec![(0, "C"), (1, "B")]),
    ];

    let sut = TransitionTable::new(&states, &alphabet, transitions);

    match sut {
        Ok(_) => panic!("Expected Err: {ERR_REDEFINED_INPUT_TRANSITION}"),
        Err(err) => assert_eq!(ERR_REDEFINED_INPUT_TRANSITION, err)
    }
}

#[test]
fn given_a_collection_of_transitions_with_undefined_states_should_get_an_err() {
    let symbols = vec![0, 1];
    let states = vec![
        StateNode::Initial("A"),
        StateNode::Interim("C"),
        StateNode::Final("B"),
    ];

    let alphabet = Alphabet::new(&symbols).expect("a valid alphabet");
    let states = States::new(&states).expect("valid states");

    let transitions = vec![
        ("A", vec![(0, "A"), (1, "C")]),
        ("C", vec![(0, "C")]),
        ("B", vec![(0, "C"), (1, "B")]),
    ];

    let sut = TransitionTable::new(&states, &alphabet, transitions);

    match sut {
        Ok(_) => panic!("Expected Err: {ERR_INCOMPLETE_INPUT_TRANSITIONS}"),
        Err(err) => assert_eq!(ERR_INCOMPLETE_INPUT_TRANSITIONS, err)
    }
}

#[test]
fn given_a_collection_of_valid_state_transitions_should_give_you_a_transition_table() {
    use Sym::{S0, S1};
    use Sta::{A, B, C};

    #[derive(Copy, Clone, Eq, PartialEq, Hash)]
    enum Sym { S0, S1 }

    #[derive(Copy, Clone, Eq, PartialEq, Hash)]
    enum Sta { A, B, C }

    let symbols = vec![S0, S1];
    let states = vec![
        StateNode::Initial(A),
        StateNode::Interim(B),
        StateNode::Final(C),
    ];

    let alphabet = Alphabet::new(&symbols).expect("a valid alphabet");
    let states = States::new(&states).expect("valid states");

    let transitions = vec![
        (A, vec![(S0, A), (S1, C)]),
        (C, vec![(S0, C), (S1, A)]),
        (B, vec![(S0, C), (S1, B)]),
    ];

    let sut = TransitionTable::new(&states, &alphabet, transitions);

    if let Err(err) = sut {
        panic!("Unexpected Err: {err}");
    }
}
