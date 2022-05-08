use crate::model::{Q, State, Σ};
use crate::nfa::delta::{
    ERR_DANGLING_STATE,
    ERR_DUPED_INPUT_TRANSITION,
    ERR_DUPED_TRANSITION,
    ERR_MISSING_FINAL_STATE_TRANSITION,
    ERR_MISSING_INITIAL_STATE_TRANSITION,
    ERR_MISSING_STATE_TRANSITION,
    ERR_UNDEFINED_SYMBOL,
    ERR_UNDEFINED_TRANSITION_STATE,
};
use crate::nfa::δ;

#[test]
fn given_a_collection_of_transitions_with_dangling_states_should_get_an_err() {
    let symbols = vec![0, 1];
    let states = vec![
        State::Initial("A"),
        State::Interim("C"),
        State::Final("B"),
    ];

    let sigma = Σ::new(&symbols).expect("valid sigma");
    let q = Q::new(&states).expect("valid states");

    let delta = vec![
        ("A", vec![(0, "A"), (0, "B")]),
        ("C", vec![(0, "C"), (1, "B")]),
        ("B", vec![(0, "A"), (1, "B")]),
    ];

    let sut = δ::new(&q, &sigma, delta);

    match sut {
        Ok(_) => panic!("Expected Err: {ERR_DANGLING_STATE}"),
        Err(err) => assert_eq!(ERR_DANGLING_STATE, err)
    }
}

#[test]
fn given_a_collection_of_transitions_with_duplicate_input_transitions_should_get_an_err() {
    let symbols = vec![0, 1];
    let states = vec![
        State::Initial("A"),
        State::Interim("C"),
        State::Final("B"),
    ];

    let sigma = Σ::new(&symbols).expect("valid sigma");
    let q = Q::new(&states).expect("valid states");

    let delta = vec![
        ("A", vec![(0, "A"), (1, "C")]),
        ("C", vec![(0, "C"), (0, "C")]),
        ("B", vec![(0, "C")]),
    ];

    let sut = δ::new(&q, &sigma, delta);

    match sut {
        Ok(_) => panic!("Expected Err: {ERR_DUPED_INPUT_TRANSITION}"),
        Err(err) => assert_eq!(ERR_DUPED_INPUT_TRANSITION, err)
    }
}

#[test]
fn given_a_collection_of_transitions_with_duplicate_states_should_get_an_err() {
    let symbols = vec![0, 1];
    let states = vec![
        State::Initial("A"),
        State::Interim("C"),
        State::Final("B"),
    ];

    let sigma = Σ::new(&symbols).expect("valid sigma");
    let q = Q::new(&states).expect("valid states");

    let delta = vec![
        ("A", vec![(0, "A"), (1, "C")]),
        ("C", vec![]),
        ("C", vec![(0, "C"), (1, "B")]),
    ];

    let sut = δ::new(&q, &sigma, delta);

    match sut {
        Ok(_) => panic!("Expected Err: {ERR_DUPED_TRANSITION}"),
        Err(err) => assert_eq!(ERR_DUPED_TRANSITION, err)
    }
}

#[test]
fn given_a_collection_of_transitions_with_missing_final_state_should_get_an_err() {
    let symbols = vec![0, 1];
    let states = vec![
        State::Initial("A"),
        State::Interim("C"),
        State::Final("B"),
    ];

    let sigma = Σ::new(&symbols).expect("valid sigma");
    let q = Q::new(&states).expect("valid states");

    let delta = vec![
        ("A", vec![(0, "A"), (1, "C")]),
        ("C", vec![(1, "A")]),
    ];

    let sut = δ::new(&q, &sigma, delta);

    match sut {
        Ok(_) => panic!("Expected Err: {ERR_MISSING_FINAL_STATE_TRANSITION}"),
        Err(err) => assert_eq!(ERR_MISSING_FINAL_STATE_TRANSITION, err)
    }
}

#[test]
fn given_a_collection_of_transitions_with_missing_initial_state_should_get_an_err() {
    let symbols = vec![0, 1];
    let states = vec![
        State::Initial("A"),
        State::Interim("C"),
        State::Final("B"),
    ];

    let sigma = Σ::new(&symbols).expect("valid sigma");
    let q = Q::new(&states).expect("valid states");

    let delta = vec![
        ("B", vec![(1, "C")]),
        ("C", vec![(0, "C"), (1, "B")]),
    ];

    let sut = δ::new(&q, &sigma, delta);

    match sut {
        Ok(_) => panic!("Expected Err: {ERR_MISSING_INITIAL_STATE_TRANSITION}"),
        Err(err) => assert_eq!(ERR_MISSING_INITIAL_STATE_TRANSITION, err)
    }
}

#[test]
fn given_a_collection_of_transitions_with_missing_input_transitions_should_get_an_err() {
    let symbols = vec![0, 1];
    let states = vec![
        State::Initial("A"),
        State::Interim("C"),
        State::Final("B"),
    ];

    let sigma = Σ::new(&symbols).expect("valid sigma");
    let q = Q::new(&states).expect("valid states");

    let delta = vec![
        ("A", vec![(0, "A"), (1, "C")]),
        ("C", vec![(0, "C"), (1, "A"), (1, "B")]),
        ("D", vec![(0, "C"), (1, "B")]),
    ];

    let sut = δ::new(&q, &sigma, delta);

    match sut {
        Ok(_) => panic!("Expected Err: {ERR_UNDEFINED_TRANSITION_STATE}"),
        Err(err) => assert_eq!(ERR_UNDEFINED_TRANSITION_STATE, err)
    }
}

#[test]
fn given_a_collection_of_transitions_with_missing_state_transitions_should_get_an_err() {
    let symbols = vec![0, 1];
    let states = vec![
        State::Initial("A"),
        State::Interim("C"),
        State::Final("B"),
    ];

    let sigma = Σ::new(&symbols).expect("valid sigma");
    let q = Q::new(&states).expect("valid states");

    let delta = vec![
        ("A", vec![(0, "A"), (1, "C")]),
        ("B", vec![]),
    ];

    let sut = δ::new(&q, &sigma, delta);

    match sut {
        Ok(_) => panic!("Expected Err: {ERR_MISSING_STATE_TRANSITION}"),
        Err(err) => assert_eq!(ERR_MISSING_STATE_TRANSITION, err)
    }
}

#[test]
fn given_a_collection_of_transitions_with_undefined_symbols_should_get_an_err() {
    let symbols = vec![0, 1];
    let states = vec![
        State::Initial("A"),
        State::Interim("C"),
        State::Final("B"),
    ];

    let sigma = Σ::new(&symbols).expect("valid sigma");
    let q = Q::new(&states).expect("valid states");

    let delta = vec![
        ("A", vec![(0, "A"), (1, "C"), (2, "B")]),
        ("C", vec![(0, "C")]),
        ("B", vec![(0, "C"), (1, "B")]),
    ];

    let sut = δ::new(&q, &sigma, delta);

    match sut {
        Ok(_) => panic!("Expected Err: {ERR_UNDEFINED_SYMBOL}"),
        Err(err) => assert_eq!(ERR_UNDEFINED_SYMBOL, err)
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
        State::Initial(SA),
        State::Interim(SB),
        State::Final(SC),
    ];

    let sigma = Σ::new(&symbols).expect("valid sigma");
    let q = Q::new(&states).expect("valid states");

    let delta = vec![
        (SA, vec![(S0, SC)]),
        (SC, vec![(S0, SB), (S0, SA)]),
        (SB, vec![]),
    ];

    let sut = δ::new(&q, &sigma, delta);

    if let Err(err) = sut {
        panic!("Unexpected Err: {err}");
    }
}
