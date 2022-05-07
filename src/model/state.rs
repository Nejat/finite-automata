use std::fmt::{Debug, Display, Formatter, Write};
use std::fmt;

#[cfg(test)]
use crate::youve_been_duped;
use crate::youve_been_duped_ref;

pub(crate) const ERR_DUPED_STATES: &str = "States must be a unique collection of state identifiers";
pub(crate) const ERR_INITIAL_STATES: &str = "States must contain at most one initial state";
pub(crate) const ERR_FINAL_STATES: &str = "States must contain at least one final state";

#[derive(Debug, Eq, PartialEq, Hash)]
pub enum State<T: Eq> {
    Initial(T),
    Interim(T),
    Final(T),
}

impl<T: Eq> AsRef<T> for State<T> {
    fn as_ref(&self) -> &T {
        match self {
            State::Initial(node) |
            State::Interim(node) |
            State::Final(node) => node
        }
    }
}

#[derive(Eq, PartialEq, Hash)]
pub struct Tag<'a, T: Eq>(Vec<&'a T>);

#[cfg(test)]
impl<'a, T: Eq> Tag<'a, T> {
    pub(crate) fn new(sub_states: &'a [T]) -> Result<Self, &'static str> {
        if youve_been_duped(sub_states) {
            Err("State must be a unique collection of values")
        } else {
            Ok(Self(sub_states.iter().collect()))
        }
    }
}

impl<'a, T: Eq> AsRef<[&'a T]> for Tag<'a, T> {
    fn as_ref(&self) -> &[&'a T] {
        &self.0
    }
}

impl<'a, T: Display + Eq> Debug for Tag<'a, T> {
    fn fmt(&self, fmt: &mut Formatter<'_>) -> fmt::Result {
        fmt.write_char('{')?;
        for itm in self.0.iter().take(1) {
            fmt.write_fmt(format_args!("{itm}"))?;
        }
        for itm in self.0.iter().skip(1) {
            fmt.write_fmt(format_args!(",{itm}"))?;
        }
        fmt.write_char('}')
    }
}

impl<'a, T: Display + Eq> Display for Tag<'a, T> {
    fn fmt(&self, fmt: &mut Formatter<'_>) -> fmt::Result {
        for itm in self.0.iter() {
            fmt.write_fmt(format_args!("{itm}"))?;
        }

        Ok(())
    }
}

pub struct Q<'a, T: Eq>(Vec<State<Tag<'a, T>>>);

impl<'a, T: Eq> Q<'a, T> {
    pub(crate) fn get_state(&self, state: T) -> Option<&State<Tag<'a, T>>> {
        let find = vec![&state];

        match self.as_ref().iter().find(|v| (**v).as_ref().as_ref() == find) {
            Some(state) => Some(state),
            None => None
        }
    }
}

impl<'a, T: Eq> Q<'a, T> {
    pub fn new(states: &'a [State<T>]) -> Result<Self, &'static str> {
        if youve_been_duped_ref(states) {
            Err(ERR_DUPED_STATES)
        } else {
            let initial_states = states.iter().filter(|v| matches!(v, State::Initial(_))).count();

            if initial_states != 1 {
                Err(ERR_INITIAL_STATES)
            } else {
                let final_states = states.iter().filter(|v| matches!(v, State::Final(_))).count();

                if final_states == 0 {
                    Err(ERR_FINAL_STATES)
                } else {
                    Ok(Self(states.iter().map(
                        |v| match v {
                            State::Initial(node) => State::Initial(Tag(vec![node])),
                            State::Interim(node) => State::Interim(Tag(vec![node])),
                            State::Final(node) => State::Final(Tag(vec![node]))
                        }
                    ).collect()))
                }
            }
        }
    }
}

impl<'a, T: Eq> AsRef<[State<Tag<'a, T>>]> for Q<'a, T> {
    fn as_ref(&self) -> &[State<Tag<'a, T>>] {
        &self.0
    }
}
