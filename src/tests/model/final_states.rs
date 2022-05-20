use crate::model::F;
use crate::model::final_states::{ERR_DUPLICATE_FINAL_STATES, ERR_EMPTY_FINAL_STATES};
use crate::tests::{assert_err, VALID_FINAL_STATES};

#[test]
fn given_a_collection_of_one_final_state_should_be_ok() {
    F::new(vec!['A']).expect(VALID_FINAL_STATES);
}

#[test]
fn given_a_collection_of_unique_final_states_should_be_ok() {
    F::new(vec!['A', 'B', 'C']).expect(VALID_FINAL_STATES);
}

#[test]
fn given_a_collection_with_duped_final_states_should_get_an_err() {
    let actual = F::new(vec!['A', 'B', 'C', 'B']); // ((B)) is defined twice

    assert_err(ERR_DUPLICATE_FINAL_STATES, &actual);
}

#[test]
fn given_an_empty_collection_of_final_states_should_get_an_err() {
    let actual = F::new(<Vec<char>>::new());

    assert_err(ERR_EMPTY_FINAL_STATES, &actual);
}
