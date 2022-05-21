use std::fmt::{Debug, Display, Formatter, Write};
use std::fmt;

mod automata;
mod model;
// mod nfa;

const STEPS_NO_ERRORS: &str = "expect no errors in steps";

const VALID_DELTA: &str = "valid δ";
const VALID_FINAL_STATES: &str = "valid final states";
const VALID_SIGMA: &str = "valid Σ";
const VALID_STATES: &str = "valid Q";

#[derive(Eq, PartialEq)]
pub enum MachineState { A, B }

impl Debug for MachineState {
    fn fmt(&self, fmt: &mut Formatter<'_>) -> fmt::Result {
        fmt.write_char(
            match self {
                Self::A => 'A',
                Self::B => 'B'
            }
        )
    }
}

impl Display for MachineState {
    fn fmt(&self, fmt: &mut Formatter<'_>) -> fmt::Result {
        fmt.write_char(
            match self {
                Self::A => 'A',
                Self::B => 'B'
            }
        )
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub enum Sym { S0, S1 }

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub enum Sta { SA, SB, SC, SD }

pub fn assert_err<T>(expected: &str, actual: &Result<T, &str>) {
    match actual {
        // actual > Ok > Panic - only happens on actual failiure, can't include in cover
        Ok(_) => panic!("Expected Err: {expected}"),
        Err(actual) => assert_eq!(expected, *actual)
    }
}

#[test]
#[should_panic]
fn given_no_way_to_supress_code_coverage_for_untestable_test_logic_we_should_devise_a_useless_test_to_accommplish_code_coverage() {
    assert_err("this is foolish", &Ok(()));
}
