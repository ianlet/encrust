#![allow(dead_code)]

extern crate regex;

mod suite;
mod feature;
mod scenario;

mod context;
mod step;
mod step_definition;
mod pattern;

#[cfg(test)]
#[macro_use]
extern crate hamcrest;

#[cfg(test)]
mod testing;
