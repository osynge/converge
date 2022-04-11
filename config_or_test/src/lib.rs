//! This crate provides a convenient concise way to write unit tests for
//! implementations of [config_or::ConfigOr].
#[cfg_attr(test, macro_use)]
extern crate config_or_derive;
mod core;
mod generic;
mod nested;
mod not_optional;
