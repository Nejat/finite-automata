#![allow(non_snake_case)]

use crate::automata::NFA;
use crate::model::{F, Q, δ, Σ};
use crate::tests::{VALID_DELTA, VALID_FINAL_STATES, VALID_SIGMA, VALID_STATES};

#[allow(clippy::module_inception)]
mod nfa;
mod nfa_configuration;
mod nfa_with_deterministic_detlas;

const VALID_NFA: &str = "valid nfa";

fn assert_steps_case(
    tags: Vec<char>,
    case: Vec<(char, Vec<(u8, char)>)>,
    expected: bool,
    steps: fn(nfa: &mut NFA<u8, char>),
) {
    let Σ = Σ::new(vec![0, 1]).expect(VALID_SIGMA);
    let Q = Q::new(tags).expect(VALID_STATES);
    let δ = δ::new(case).expect(VALID_DELTA);
    let F = F::new(vec!['B']).expect(VALID_FINAL_STATES);
    let mut sut = NFA::new(Q, Σ, δ, 'A', &F).expect(VALID_NFA);

    steps(&mut sut);

    let actual = sut.matches();

    assert_eq!(expected, actual);
}
