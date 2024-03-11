#![feature(extract_if)]
#![feature(iterator_try_collect)]
#![feature(fn_traits)]
#![warn(clippy::all, rust_2018_idioms)]

#[macro_use]
extern crate derivative;

pub mod backend;
pub mod units;
