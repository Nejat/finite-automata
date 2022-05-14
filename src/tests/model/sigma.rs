use crate::model::sigma::ERR_DUPED_ALPHABET;
use crate::model::Σ;
use crate::tests::assert_err;

const VALID_ALPHABET: &str = "a valid alphabet";

#[test]
fn given_a_collection_of_symbols_with_dupes_we_should_get_an_err() {
    let sut = Σ::new(&[10, 20, 30, 10, 50]);

    assert_err(ERR_DUPED_ALPHABET, &sut);
}

#[test]
fn given_a_collection_of_unique_symbols_we_should_get_symbols() {
    let sut = Σ::new(&[10, 20, 30, 40, 50]).expect(VALID_ALPHABET);

    assert_eq!(sut.as_ref(), &[10, 20, 30, 40, 50]);
}
