use std::fmt::{Debug, Display};

use crate::model::Q;
use crate::model::State;
use crate::model::state::{
    ERR_DUPLICATE_STATES, ERR_DUPLICATE_TAGS,
    ERR_EMPTY_STATES, ERR_EMPTY_TAGS, Phase,
};
use crate::tests::{assert_err, MachineState, VALID_STATES};

const VALID_STATE: &str = "a valid state";

#[test]
fn given_a_state_with_more_than_one_tag_should_debug_brackets_and_comma_seperated_values() {
    let states = vec![&MachineState::A, &MachineState::B];

    assert_debug_of_all_phases(&states, "A,B");
}

#[test]
fn given_a_state_with_more_than_one_tag_should_display_joined_values() {
    let states = vec![&MachineState::A, &MachineState::B];

    assert_display_of_all_phases(&states, "AB");
}

#[test]
fn given_a_state_with_no_tags_we_should_get_an_err() {
    let states = <Vec<&str>>::new();
    let sut = State::new(states, Phase::Interim);

    assert_err(ERR_EMPTY_TAGS, &sut);
}

#[test]
fn given_a_state_with_duplicate_tags_we_should_get_an_err() {
    let states = vec!['A', 'A']; // duplicate tags
    let sut = State::new(states, Phase::Interim);

    assert_err(ERR_DUPLICATE_TAGS, &sut);
}

#[test]
fn given_a_state_with_one_tag_should_debug_brackets_and_the_value() {
    let states = vec![&MachineState::A];

    assert_debug_of_all_phases(&states, "A");
}

#[test]
fn given_a_state_with_one_tag_should_display_the_value() {
    let states = vec![&MachineState::A];

    assert_display_of_all_phases(&states, "A");
}

#[test]
#[allow(non_snake_case)]
fn given_a_collection_of_unique_states_we_should_get_a_set_of_states_Q() {
    Q::new(vec!['A']).expect(VALID_STATES);
    Q::new(vec!['A', 'B', 'C']).expect(VALID_STATES);
}

#[test]
fn given_an_empty_collection_of_states_we_should_get_an_err() {
    let states = <Vec<()>>::new(); // no states defined
    let sut = Q::new(states);

    assert_err(ERR_EMPTY_STATES, &sut);
}

#[test]
fn given_states_with_dupes_we_should_get_an_err() {
    let sut = Q::new(vec!['A', 'B', 'A']);

    assert_err(ERR_DUPLICATE_STATES, &sut);
}

#[test]
fn given_necessary_derives_we_cover_them_too() {
    // two for one
    assert_eq!(&format!("{:?}", Phase::Initial), &format!("{:?}", Phase::Initial.clone()), "clone");
}

fn assert_debug_of_all_phases<S>(source: &[S], expected: &str)
    where S: Clone + Debug + Display + Eq
{
    assert_output_of_all_phases(source, |pre, pst| format!("{pre}{{{expected}}}{pst}"), |sut| format!("{sut:?}"));
}

fn assert_display_of_all_phases<S>(source: &[S], expected: &str)
    where S: Clone + Display + Eq
{
    assert_output_of_all_phases(source, move |pre, pst| format!("{pre}{expected}{pst}"), |sut| format!("{sut}"));
}

fn assert_output_of_all_phases<S, E>(source: &[S], expected: E, actual: fn(&State<S>) -> String)
    where S: Clone + Display + Eq,
          E: Fn(&str, &str) -> String
{
    let phases = vec![
        (Phase::Initial, ">(", ")"),
        (Phase::Interim, "(", ")"),
        (Phase::Final, "((", "))"),
        (Phase::Both, ">((", "))"),
    ];

    for (phase, prefix, postfix) in phases {
        let states = source.to_vec();
        let sut = State::new(states, phase).expect(VALID_STATE);
        let expected = expected(prefix, postfix);
        let actual = actual(&sut);

        assert_eq!(expected, actual, "{:?} State Phase", phase);
    }
}
