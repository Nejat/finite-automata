use std::fmt::{Debug, Display, Formatter, Write};
use std::fmt;

use crate::youve_been_duped;

pub const ERR_DUPED_STATES: &str = "States must be a unique collection";
pub const ERR_DUPED_TAGS: &str = "State must be a unique collection of tags";
pub const ERR_EMPTY_STATES: &str = "States must contain at least on state";
pub const ERR_EMPTY_TAGS: &str = "State must contain at most one tag";
pub const ERR_INITIAL_STATE: &str = "States must contain at most one initial state";
pub const ERR_FINAL_STATES: &str = "States must contain at least one final state";
pub const ERR_UNDEFINED_FINAL_STATE: &str = "Final state f is not difined in Q";
pub const ERR_UNDEFINED_INITIAL_STATE: &str = "Initial state q0 is not defined in Q";

///
#[repr(u8)]
#[derive(Eq, PartialEq, Hash)]
pub enum Phase {
    ///
    Initial,

    ///
    Interim,

    ///
    Final,

    ///
    Both,
}

///
#[derive(Eq, PartialEq, Hash)]
pub struct State<'a, S: Eq> {
    tags: Vec<&'a S>,
    phase: Phase,
}

impl<'a, S: Eq> State<'a, S> {
    /// # Errors
    pub(crate) fn new(tags: Vec<&'a S>) -> Result<Self, &'static str> {
        if tags.is_empty() {
            Err(ERR_EMPTY_TAGS)
        } else if youve_been_duped(&tags) {
            Err(ERR_DUPED_TAGS)
        } else {
            Ok(Self {
                tags,
                phase: Phase::Interim,
            })
        }
    }

    #[inline]
    pub(crate) fn is_final(&self) -> bool {
        matches!(self.phase, Phase::Final | Phase::Both)
    }

    #[inline]
    pub(crate) fn is_initial(&self) -> bool {
        matches!(self.phase, Phase::Initial | Phase::Both)
    }

    pub(crate) fn set_phase(&mut self, value: Phase) {
        self.phase = match value {
            Phase::Initial if self.is_final() => Phase::Both,
            Phase::Final if self.is_initial() => Phase::Both,
            phase => phase
        };
    }
}

impl<'a, S: Eq> AsRef<[&'a S]> for State<'a, S> {
    fn as_ref(&self) -> &[&'a S] {
        &self.tags
    }
}

impl<'a, S: Display + Eq> Debug for State<'a, S> {
    fn fmt(&self, fmt: &mut Formatter<'_>) -> fmt::Result {
        write_start_phase(&self.phase, fmt)?;
        fmt.write_char('{')?;

        for itm in self.tags.iter().take(1) {
            fmt.write_fmt(format_args!("{itm}"))?;
        }

        for itm in self.tags.iter().skip(1) {
            fmt.write_fmt(format_args!(",{itm}"))?;
        }

        fmt.write_char('}')?;
        write_end_phase(&self.phase, fmt)
    }
}

impl<'a, S: Display + Eq> Display for State<'a, S> {
    fn fmt(&self, fmt: &mut Formatter<'_>) -> fmt::Result {
        write_start_phase(&self.phase, fmt)?;

        for itm in &self.tags {
            fmt.write_fmt(format_args!("{itm}"))?;
        }

        write_end_phase(&self.phase, fmt)
    }
}

fn write_end_phase(phase: &Phase, fmt: &mut Formatter<'_>) -> fmt::Result {
    match phase {
        Phase::Initial |
        Phase::Interim => fmt.write_char(')'),
        Phase::Final |
        Phase::Both => fmt.write_str("))")
    }
}

fn write_start_phase(phase: &Phase, fmt: &mut Formatter<'_>) -> fmt::Result {
    match phase {
        Phase::Initial => fmt.write_str(">("),
        Phase::Interim => fmt.write_char('('),
        Phase::Final => fmt.write_str("(("),
        Phase::Both => fmt.write_str(">((")
    }
}

///
pub struct Q<'a, S: Eq>(Vec<State<'a, S>>);

impl<'a, S: Eq> Q<'a, S> {
    /// # Errors
    #[allow(clippy::missing_panics_doc)] // unwrap is ok, check below
    pub fn new(states: &'a [S]) -> Result<Self, &'static str> {
        if states.is_empty() {
            Err(ERR_EMPTY_STATES)
        } else if youve_been_duped(states) {
            Err(ERR_DUPED_STATES)
        } else {
            let states = states.iter()
                // unwrap is ok because tags is never empty or duped here
                .map(|tag| State::new(vec![tag]).unwrap())
                .collect::<Vec<_>>();

            Ok(Self(states))
        }
    }

    #[inline]
    pub(crate) fn get_state(&'a self, state: &[&S]) -> Option<&'a State<'a, S>> {
        self.0.iter().find(|q| q.as_ref() == state)
    }

    #[inline]
    pub(crate) fn get_state_mut(&mut self, state: &[&S]) -> Option<&mut State<'a, S>> {
        self.0.iter_mut().find(|q| q.as_ref() == state)
    }

    #[allow(non_snake_case)]
    pub(crate) fn set_phases(&mut self, q0: &S, F: &[S],
    ) -> Result<(), &'static str> {
        let initial = self.get_state_mut(&[q0]).ok_or(ERR_UNDEFINED_INITIAL_STATE)?;

        initial.set_phase(Phase::Initial);

        for f in F {
            let fin = self.get_state_mut(&[f]).ok_or(ERR_UNDEFINED_FINAL_STATE)?;

            fin.set_phase(Phase::Final);
        }

        if self.0.iter().filter(|state| state.is_initial()).count() != 1 {
            Err(ERR_INITIAL_STATE)
        } else if self.0.iter().filter(|state| state.is_final()).count() == 0 {
            Err(ERR_FINAL_STATES)
        } else {
            Ok(())
        }
    }
}

impl<'a, S: Eq> AsRef<[State<'a, S>]> for Q<'a, S> {
    fn as_ref(&self) -> &[State<'a, S>] {
        &self.0
    }
}

impl<'a, S: Debug + Display + Eq> Debug for Q<'a, S> {
    fn fmt(&self, fmt: &mut Formatter<'_>) -> fmt::Result {
        fmt.write_char('[')?;
        for itm in self.0.iter().take(1) {
            fmt.write_fmt(format_args!("{itm:?}"))?;
        }
        for itm in self.0.iter().skip(1) {
            fmt.write_fmt(format_args!(",{itm:?}"))?;
        }
        fmt.write_char(']')
    }
}

impl<'a, S: Display + Eq> Display for Q<'a, S> {
    fn fmt(&self, fmt: &mut Formatter<'_>) -> fmt::Result {
        fmt.write_char('[')?;
        for itm in self.0.iter().take(1) {
            fmt.write_fmt(format_args!("{itm}"))?;
        }
        for itm in self.0.iter().skip(1) {
            fmt.write_fmt(format_args!(",{itm}"))?;
        }
        fmt.write_char(']')
    }
}
