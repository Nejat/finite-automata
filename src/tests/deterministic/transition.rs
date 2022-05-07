use crate::deterministic::alphabet::Alphabet;
use crate::deterministic::state::{StateNode, States};
use crate::deterministic::transition::{
    ERR_DANGLING_STATE,
    ERR_DUPED_INPUT_TRANSITION,
    ERR_DUPED_TRANSITION,
    ERR_INCOMPLETE_INPUT_TRANSITIONS,
    ERR_MISSING_FINAL_STATE_TRANSITION,
    ERR_MISSING_INITIAL_STATE_TRANSITION,
    ERR_MISSING_STATE_TRANSITION,
    ERR_REDEFINED_INPUT_TRANSITION,
    ERR_UNDEFINED_TRANSITION_STATE,
    TransitionTable
};

#[test]
fn given_a_collection_of_transitions_with_dangling_states_should_get_an_err() {
    let symbols = vec![0, 1];
    let states = vec![
        StateNode::Initial("A"),
        StateNode::Interim("C"),
        StateNode::Final("B"),
    ];

    let alphabet = Alphabet::new(&symbols).expect("a valid alphabet");
    let states = States::new(&states).expect("valid states");

    let transitions = vec![
        ("A", vec![(0, "A"), (1, "B")]),
        ("C", vec![(0, "C"), (1, "B")]),
        ("B", vec![(0, "A"), (1, "B")]),
    ];

    let sut = TransitionTable::new(&states, &alphabet, transitions);

    match sut {
        Ok(_) => panic!("Expected Err: {ERR_DANGLING_STATE}"),
        Err(err) => assert_eq!(ERR_DANGLING_STATE, err)
    }
}

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
fn given_a_collection_of_transitions_with_missing_final_state_should_get_an_err() {
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
    ];

    let sut = TransitionTable::new(&states, &alphabet, transitions);

    match sut {
        Ok(_) => panic!("Expected Err: {ERR_MISSING_FINAL_STATE_TRANSITION}"),
        Err(err) => assert_eq!(ERR_MISSING_FINAL_STATE_TRANSITION, err)
    }
}

#[test]
fn given_a_collection_of_transitions_with_missing_initial_state_should_get_an_err() {
    let symbols = vec![0, 1];
    let states = vec![
        StateNode::Initial("A"),
        StateNode::Interim("C"),
        StateNode::Final("B"),
    ];

    let alphabet = Alphabet::new(&symbols).expect("a valid alphabet");
    let states = States::new(&states).expect("valid states");

    let transitions = vec![
        ("B", vec![(0, "B"), (1, "C")]),
        ("C", vec![(0, "C"), (1, "B")]),
    ];

    let sut = TransitionTable::new(&states, &alphabet, transitions);

    match sut {
        Ok(_) => panic!("Expected Err: {ERR_MISSING_INITIAL_STATE_TRANSITION}"),
        Err(err) => assert_eq!(ERR_MISSING_INITIAL_STATE_TRANSITION, err)
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
        ("B", vec![(0, "C"), (1, "B")]),
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
    use Sta::{SA, SB, SC};

    #[derive(Copy, Clone, Eq, PartialEq, Hash)]
    enum Sym { S0, S1 }

    #[derive(Copy, Clone, Eq, PartialEq, Hash)]
    enum Sta { SA, SB, SC }

    let symbols = vec![S0, S1];
    let states = vec![
        StateNode::Initial(SA),
        StateNode::Interim(SB),
        StateNode::Final(SC),
    ];

    let alphabet = Alphabet::new(&symbols).expect("a valid alphabet");
    let states = States::new(&states).expect("valid states");

    let transitions = vec![
        (SA, vec![(S0, SA), (S1, SB)]),
        (SC, vec![(S0, SC), (S1, SA)]),
        (SB, vec![(S0, SC), (S1, SB)]),
    ];

    let sut = TransitionTable::new(&states, &alphabet, transitions);

    if let Err(err) = sut {
        panic!("Unexpected Err: {err}");
    }
}
