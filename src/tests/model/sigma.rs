use crate::model::sigma::{ERR_DUPLICATE_ALPHABET, ERR_EMPTY_ALPHABET};
use crate::model::Σ;
use crate::tests::{assert_err, VALID_SIGMA};

#[test]
fn given_a_collection_of_symbols_with_dupes_we_should_get_an_err() {
    let sut = Σ::new(vec![10, 20, 30, 10, 50]); // symbol 10 is defined twice

    assert_err(ERR_DUPLICATE_ALPHABET, &sut);
}

#[test]
fn given_a_collection_of_unique_symbols_we_should_get_symbols() {
    let sut = Σ::new(vec![10, 20, 30, 40, 50]).expect(VALID_SIGMA);

    assert_eq!(&*sut, &[10, 20, 30, 40, 50]);
}

#[test]
fn given_an_empty_collection_of_symbol_swe_should_get_an_err() {
    let sut = Σ::new(<Vec<u8>>::new());

    assert_err(ERR_EMPTY_ALPHABET, &sut);
}
