use crate::model::{Q, Σ};
use crate::nfa::NFA;
use crate::tests::{VALID_DELTA, VALID_SIGMA, VALID_STATES};

mod nfa;
mod nfa_with_deterministic_detlas;

fn assert_steps_case(
    states: &[char],
    case: Vec<(char, Vec<(u8, char)>)>,
    expected: bool, steps: fn(nfa: &mut NFA<u8, char>),
) {
    let sigma = Σ::new(vec![0, 1]).expect(VALID_SIGMA);
    let mut q = Q::new(states).expect(VALID_STATES);
    let delta = crate::nfa::δ::new(&mut q, sigma, case, 'A', vec!['B']).expect(VALID_DELTA);
    let mut sut = NFA::new(delta);

    steps(&mut sut);

    let actual = sut.matches();

    assert_eq!(expected, actual);
}
