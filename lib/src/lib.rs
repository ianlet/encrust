#![allow(dead_code)]

extern crate regex;

mod context;
mod pattern;
mod step;
mod step_definition;

#[cfg(test)]
#[macro_use]
extern crate hamcrest;

#[cfg(test)]
mod testing;
