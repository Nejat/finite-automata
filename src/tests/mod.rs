use std::fmt::{Debug, Display, Formatter, Write};
use std::fmt;

mod dfa;
mod model;
mod nfa;

#[derive(Eq, PartialEq)]
pub enum MachineState { A, B }

impl Display for MachineState {
    fn fmt(&self, fmt: &mut Formatter<'_>) -> fmt::Result {
        fmt.write_char(
            match self {
                MachineState::A => 'A',
                MachineState::B => 'B'
            }
        )
    }
}

impl Debug for MachineState {
    fn fmt(&self, fmt: &mut Formatter<'_>) -> fmt::Result {
        fmt.write_char(
            match self {
                MachineState::A => 'A',
                MachineState::B => 'B'
            }
        )
    }
}
