use std::borrow::Borrow;
use std::fmt::{Debug, Display, Formatter, Write};
use std::fmt;
use std::hash::{Hash, Hasher};
use std::ops::Deref;
use std::rc::Rc;

use crate::utils::duped::Duped;

pub const ERR_DUPLICATE_STATES: &str = "States must be a unique collection";
pub const ERR_DUPLICATE_TAGS: &str = "State must be a unique collection of tags";
pub const ERR_EMPTY_STATES: &str = "States must contain at least one state";
pub const ERR_EMPTY_TAGS: &str = "State must contain at least one tag";

// todo: code coverage reports missing coverage for phase derives???
///
#[repr(u8)]
#[derive(Copy, Clone)]
#[cfg_attr(test, derive(Debug))]
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
pub struct State<S> {
    tags: Rc<Vec<S>>,
    phase: Phase,
}

impl<S: Eq> Eq for State<S> {}

impl<S: Eq> PartialEq for State<S> {
    fn eq(&self, other: &Self) -> bool {
        self.tags == other.tags
    }
}

impl<S: Hash> Hash for State<S> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.tags.hash(state);
    }
}

impl<S> Deref for State<S> {
    type Target = [S];

    fn deref(&self) -> &Self::Target {
        &self.tags
    }
}

impl<S> Borrow<Vec<S>> for State<S> {
    fn borrow(&self) -> &Vec<S> {
        &self.tags
    }
}

impl<S> Clone for State<S> {
    fn clone(&self) -> Self {
        Self {
            tags: self.tags.clone(),
            phase: self.phase,
        }
    }
}

impl<S: Eq> State<S> {
    /// # Errors
    pub(crate) fn new(tags: Vec<S>, phase: Phase) -> Result<Self, &'static str> {
        if tags.is_empty() {
            Err(ERR_EMPTY_TAGS)
        } else if tags.iter().has_dupes() {
            Err(ERR_DUPLICATE_TAGS)
        } else {
            Ok(Self {
                tags: Rc::new(tags),
                phase,
            })
        }
    }

    #[inline]
    pub(crate) const fn is_final(&self) -> bool {
        matches!(self.phase, Phase::Final | Phase::Both)
    }

    #[inline]
    pub(crate) const fn is_initial(&self) -> bool {
        matches!(self.phase, Phase::Initial | Phase::Both)
    }
}

impl<S: Debug> Debug for State<S> {
    fn fmt(&self, fmt: &mut Formatter<'_>) -> fmt::Result {
        write_start_phase(self.phase, fmt)?;
        fmt.write_char('{')?;

        for itm in self.tags.iter().take(1) {
            fmt.write_fmt(format_args!("{itm:?}"))?;
        }

        for itm in self.tags.iter().skip(1) {
            fmt.write_fmt(format_args!(",{itm:?}"))?;
        }

        fmt.write_char('}')?;
        write_end_phase(self.phase, fmt)
    }
}

impl<S: Display> Display for State<S> {
    fn fmt(&self, fmt: &mut Formatter<'_>) -> fmt::Result {
        write_start_phase(self.phase, fmt)?;

        for itm in self.tags.iter() {
            fmt.write_fmt(format_args!("{itm}"))?;
        }

        write_end_phase(self.phase, fmt)
    }
}

/// Set of all states
pub struct Q<S>(Vec<S>);

impl<S: Eq> Q<S> {
    /// # Errors
    pub fn new(states: Vec<S>) -> Result<Self, &'static str> {
        if states.is_empty() {
            Err(ERR_EMPTY_STATES)
        } else if states.iter().has_dupes() {
            Err(ERR_DUPLICATE_STATES)
        } else {
            Ok(Self(states))
        }
    }
}

impl<S> Deref for Q<S> {
    type Target = Vec<S>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<S> From<Q<S>> for Vec<S> {
    fn from(source: Q<S>) -> Self {
        source.0
    }
}

fn write_end_phase(phase: Phase, fmt: &mut Formatter<'_>) -> fmt::Result {
    match phase {
        Phase::Initial |
        Phase::Interim => fmt.write_char(')'),
        Phase::Final |
        Phase::Both => fmt.write_str("))")
    }
}

fn write_start_phase(phase: Phase, fmt: &mut Formatter<'_>) -> fmt::Result {
    match phase {
        Phase::Initial => fmt.write_str(">("),
        Phase::Interim => fmt.write_char('('),
        Phase::Final => fmt.write_str("(("),
        Phase::Both => fmt.write_str(">((")
    }
}
