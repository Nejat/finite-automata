use crate::model::{Q, State};
use crate::model::state::{ERR_DUPED_STATES, ERR_EMPTY_STATES, ERR_EMPTY_TAGS, ERR_FINAL_STATES, ERR_INITIAL_STATE};
use crate::tests::{assert_err, MachineState};

const VALID_INITIAL_STATES: &str = "valid initial states";
const VALID_PHASES: &str = "valid phases";
const VALID_STATE: &str = "a valid state";

// todo refactor tests to public interface, i.e. State::new is not public

#[test]
fn given_a_collection_of_unique_values_for_a_state_we_should_get_a_state() {
    let states = vec![&'A', &'B'];
    let sut = State::new(states).expect(VALID_STATE);
    let expected = &[&'A', &'B'];
    let actual = sut.as_ref();

    assert_eq!(expected, actual);
}

#[test]
fn given_a_valid_state_should_debug_output_comma_delimited_states_with_phase() {
    let states = vec!['A', 'C', 'B'];
    let mut sut = Q::new(&states).expect(VALID_INITIAL_STATES);

    sut.set_phases(&'A', &['B']).expect(VALID_PHASES);

    let expected = "[>({A}),({C}),(({B}))]";

    assert_eq!(expected, format!("{sut:?}"));
}

#[test]
fn given_a_valid_state_with_initial_state_also_final_should_debug_output_comma_delimited_states_with_phase() {
    let states = vec!['A', 'C', 'B'];
    let mut sut = Q::new(&states).expect(VALID_INITIAL_STATES);

    sut.set_phases(&'A', &['A', 'B']).expect(VALID_PHASES);

    let expected = "[>(({A})),({C}),(({B}))]";

    assert_eq!(expected, format!("{sut:?}"));
}

#[test]
fn given_a_collection_of_unique_values_for_states_should_display_output() {
    let states = vec!['A', 'C', 'B'];
    let mut sut = Q::new(&states).expect(VALID_INITIAL_STATES);

    sut.set_phases(&'A', &['B']).expect(VALID_PHASES);

    let expected = "[>(A),(C),((B))]";

    assert_eq!(expected, format!("{sut}"));
}

#[test]
fn given_a_collection_of_unique_values_for_states_we_should_get_states() {
    let states = vec!['A', 'C', 'B'];

    let sut = Q::new(&states).expect(VALID_INITIAL_STATES);

    let expected = vec![
        State::new(vec![&'A']).unwrap(),
        State::new(vec![&'C']).unwrap(),
        State::new(vec![&'B']).unwrap(),
    ];

    assert_eq!(&expected, sut.as_ref());
}

#[test]
fn given_an_empty_collection_of_states_we_should_get_an_err() {
    let states = <Vec<()>>::new();

    let sut = Q::new(&states);

    assert_err(ERR_EMPTY_STATES, &sut);
}

#[test]
fn given_a_collection_of_values_for_states_with_dupe_state_names_we_should_get_an_err() {
    let states = vec!['A', 'B', 'A'];

    let sut = Q::new(&states);

    assert_err(ERR_DUPED_STATES, &sut);
}

#[test]
fn given_a_state_with_more_than_one_value_should_debug_brackets_and_comma_seperated_values() {
    let states = vec![&MachineState::A, &MachineState::B];
    let sut = State::new(states).expect(VALID_STATE);
    let expected = "({A,B})";
    let actual = format!("{sut:?}");

    assert_eq!(expected, actual);
}

#[test]
fn given_a_state_with_no_values_we_should_get_an_err() {
    let states = vec![];
    let sut = <State<&str>>::new(states);

    assert_err(ERR_EMPTY_TAGS, &sut);
}

#[test]
fn given_a_state_with_one_value_should_debug_brackets_and_the_value() {
    let states = vec![&'A'];
    let sut = State::new(states).expect(VALID_STATE);
    let expected = "({A})";
    let actual = format!("{sut:?}");

    assert_eq!(expected, actual);
}

#[test]
fn given_a_state_with_more_than_one_value_should_display_joined_values() {
    let states = vec![&MachineState::A, &MachineState::B];
    let sut = State::new(states).expect(VALID_STATE);
    let expected = "(AB)";
    let actual = format!("{sut}");

    assert_eq!(expected, actual);
}

#[test]
fn given_a_state_with_one_value_should_display_the_value() {
    let states = vec![&'A'];
    let sut = State::new(states).expect(VALID_STATE);
    let expected = "(A)";
    let actual = format!("{sut}");

    assert_eq!(expected, actual);
}

#[test]
fn given_a_collection_of_values_for_states_with_multiple_initial_states_we_should_get_an_err() {
    let states = vec!['A', 'B', 'C', 'D'];

    let mut sut = Q::new(&states).expect(VALID_INITIAL_STATES);

    sut.set_phases(&'A', &['D']).expect(VALID_PHASES);

    let actual = sut.set_phases(&'C', &[]);

    assert_err(ERR_INITIAL_STATE, &actual);
}

#[test]
fn given_a_collection_of_values_for_states_with_no_final_states_we_should_get_an_err() {
    let states = vec!['A', 'B'];

    let mut sut = Q::new(&states).expect(VALID_INITIAL_STATES);

    let actual = sut.set_phases(&'A', &[]);

    assert_err(ERR_FINAL_STATES, &actual);
}
