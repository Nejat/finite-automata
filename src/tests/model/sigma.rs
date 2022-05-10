use crate::model::sigma::ERR_DUPED_ALPHABET;
use crate::model::Σ;

#[test]
fn given_a_collection_of_symbols_with_dupes_we_should_get_an_err() {
    let sut = Σ::new(&[10, 20, 30, 10, 50]);

    assert!(matches!(sut, Err(err) if err == ERR_DUPED_ALPHABET));
}

#[test]
fn given_a_collection_of_unique_symbols_we_should_get_symbols() {
    let sut = Σ::new(&[10, 20, 30, 40, 50]).expect("a valid alphabet");

    assert_eq!(sut.as_ref(), &[10, 20, 30, 40, 50]);
}
