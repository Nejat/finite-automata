#![deny(clippy::all)]
#![deny(clippy::pedantic)]
#![deny(clippy::nursery)]
#![deny(clippy::cargo)]
#![deny(missing_docs)]
// ==============================================================
#![deny(unused_imports)]
// ==============================================================
#![allow(clippy::module_name_repetitions)]
#![allow(clippy::items_after_statements)]
// ==============================================================
#![doc(html_root_url = "https://docs.rs/finite-state-machine/0.1.0")]

//!

extern crate core;

pub mod automata;
pub mod model;
mod utils;

#[cfg(test)]
mod tests;

// used for validated data
pub(crate) const UNREACHABLE_ERR: &str = "UNREACHABLE";
