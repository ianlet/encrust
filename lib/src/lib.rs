#![allow(dead_code)]

extern crate regex;

mod context;
mod feature;
mod pattern;
mod scenario;
mod step;
mod step_definition;

#[cfg(test)]
#[macro_use]
extern crate hamcrest;

#[cfg(test)]
mod testing;
