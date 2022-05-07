use crate::deterministic::state::{ERR_DUPED_STATES, ERR_FINAL_STATES, ERR_INITIAL_STATES, State, StateNode, States};
use crate::tests::deterministic::MachineState;

#[test]
fn given_a_collection_of_values_for_a_state_with_dupes_we_should_get_an_err() {
    let states = vec!["A", "B", "A"];
    let sut = State::new(&states);

    assert!(matches!(sut, Err(_err)))
}

#[test]
fn given_a_collection_of_unique_values_for_a_state_we_should_get_a_state() {
    let states = vec!["A", "B"];
    let sut = State::new(&states).expect("a valid state");

    assert_eq!(sut.as_ref(), &[&"A", &"B"]);
}

#[test]
fn given_a_state_with_more_than_one_value_should_debug_brackets_and_comma_seperated_values() {
    let states = vec![&MachineState::A, &MachineState::B];
    let sut = State::new(&states).expect("a valid state");

    assert_eq!(format!("{sut:?}"), "{A,B}");
}

#[test]
fn given_a_state_with_no_values_should_debug_empty_brackets() {
    let states = <Vec<&str>>::new();
    let sut = State::new(&states).expect("a valid state");

    assert_eq!(format!("{sut:?}"), "{}");
}

#[test]
fn given_a_state_with_one_value_should_debug_brackets_and_the_value() {
    let states = vec!["A"];
    let sut = State::new(&states).expect("a valid state");

    assert_eq!(format!("{sut:?}"), "{A}");
}

#[test]
fn given_a_state_with_more_than_one_value_should_display_brackets_and_comma_seperated_values() {
    let states = vec![&MachineState::A, &MachineState::B];
    let sut = State::new(&states).expect("a valid state");

    assert_eq!(format!("{sut}"), "AB");
}

#[test]
fn given_a_state_with_no_values_should_display_empty_brackets() {
    let states = <Vec<&str>>::new();
    let sut = State::new(&states).expect("a valid state");

    assert_eq!(format!("{sut}"), "");
}

#[test]
fn given_a_state_with_one_value_should_display_brackets_and_the_value() {
    let states = vec!["A"];
    let sut = State::new(&states).expect("a valid state");
    let expected = "A";

    assert_eq!(expected, format!("{sut}"));
}

#[test]
fn given_a_collection_of_values_for_states_with_dupe_state_names_we_should_get_an_err() {
    let states = vec![
        StateNode::Initial("A"),
        StateNode::Interim("B"),
        StateNode::Final("A"),
    ];
    let sut = States::new(&states);

    assert!(matches!(sut, Err(err) if err == ERR_DUPED_STATES))
}

#[test]
fn given_a_collection_of_values_for_states_with_multiple_initial_states_we_should_get_an_err() {
    let states = vec![
        StateNode::Initial("A"),
        StateNode::Interim("B"),
        StateNode::Initial("C"),
        StateNode::Final("D"),
    ];
    let sut = States::new(&states);

    assert!(matches!(sut, Err(err) if err == ERR_INITIAL_STATES))
}

#[test]
fn given_a_collection_of_values_for_states_with_no_final_states_we_should_get_an_err() {
    let states = vec![
        StateNode::Initial("A"),
        StateNode::Interim("B"),
    ];
    let sut = States::new(&states);

    assert!(matches!(sut, Err(err) if err == ERR_FINAL_STATES))
}

#[test]
fn given_a_collection_of_unique_values_for_states_should_get_states() {
    let states = vec![
        StateNode::Initial("A"),
        StateNode::Interim("C"),
        StateNode::Final("B"),
    ];
    let sut = States::new(&states).expect("valid states");
    let expected = vec![
        StateNode::Initial(State::new(&["A"]).unwrap()),
        StateNode::Interim(State::new(&["C"]).unwrap()),
        StateNode::Final(State::new(&["B"]).unwrap()),
    ];

    assert_eq!(&expected, sut.as_ref());
}