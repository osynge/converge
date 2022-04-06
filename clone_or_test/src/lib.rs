//! This crate provides a convenient concise way to write unit tests for
//! implementations of [clone_or::CloneOr].
#[cfg_attr(test, macro_use)]
extern crate clone_or_derive;
mod core;
mod generic;
mod nested;
