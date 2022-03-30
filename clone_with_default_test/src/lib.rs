//! This crate provides a convenient concise way to write unit tests for
//! implementations of [`CloneWithDefault`].
extern crate clone_with_default;
#[cfg_attr(test, macro_use)]
extern crate clone_with_default_derive;
mod core;
mod nested;
