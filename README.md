# Converge

Layered configuration system for Rust applications using little code. By only
converging configuration, flexibility of the configuration source and how it
converges is under control of the user.

## Introduction

Applications which have layered configuration of command line arguments,
environment and configuration file, are what users expect. This library allows
these sources to be layered with minimal effort and code.

## Motivation

Both `The Unix philosophy` and the idea of `microservices` lead to writing lots
of applications. It is reasonable to expect these applications will take
configuration from the command line arguments, environment and often a
configuration file. As the applications mature the number of configuration
options tends to grow. A naive approach to managing this is repetitive and error
prone, and adding testing to this increases the repetition.

## About

This library leaves the user free to decide configuration file format, command
line parsing method, and has minimal dependencies. It works with structures
holding the configuration items as fields. These structures must implement
a trait with a single method that is commonly derivable for most structures.

This method works with immutable data structures, is very low in code, but with
the cost of cloning the configuration during the layering process. This is
a reasonable compromise as it usually happens only once in an applications
lifecycle.

## Quickstart

It is based on a trait `Converge` with a single method
`converge` this is in the crate `converge`.

    pub trait Converge<Rhs = Self> {
        fn converge(self, default: Rhs) -> Self;
    }

This trait can be derived using `converge_derive` as shown below.

    #[derive(Converge)]
    pub struct Config {
        pub config_file: Option<String>,
        pub loglevel: Option<i8>,
        pub xunit_local_globs: Option<Vec<String>>,
        pub environment_sk: Option<String>,
        pub environment_keys: Option<Vec<String>>,
        pub project_sk: Option<String>,
        pub project_identifier: Option<String>,
        pub project_human_name: Option<String>,
        pub run_identifier: Option<String>,
        pub run_sk: Option<String>,
        pub service_url: Option<String>,
    }

These structures populated from can be nested and can then be coupled together
to derive new instances with clear and simple prescience.

    let config_commandline : Config = parse_commandline_to_config();
    let config_file : Config = parse_file_to_config();
    let config_env : Config = parse_env_to_config();
    let cfg = config_commandline.converge(config_file).converge(config_env);

It is possible to have T typed fields that are not Optional values, but this not
usual as then `converge` just with take the left hand side value.

## Designing your configuration structure

The structure implementing the trait `Converge` should contain a logical
grouping of fields to represent each setting you may wish to use as part of your
layered configuration.

### Example deriving nested `Converge` structures

    #[derive(Converge)]
    pub struct ConfigRabbitMqCredentials {
        pub username: Option<String>,
        pub password: Option<String>,
    }

    #[derive(Converge)]
    pub struct ConfigRabbitMQ {
        pub host: Option<String>,
        pub port: Option<i32>,
        #[converge(nest)]
        pub credentials: Option<ConfigRabbitMqCredentials>,
    }

When using Fields types that also implement `Converge` you can mark fields as
also supporting `Converge` with the `#[converge(nest)]` attribute, this then
allows `converge` to be used on this structure by `converge`.

### Deriving with custom field converge strategies

    #[derive(Converge)]
    pub struct ConfigRabbitMqCredentials {
        pub username: Option<String>,
        pub password: Option<String>,
    }

    #[derive(Converge)]
    pub struct ConfigRabbitMQ {
        pub host: Option<String>,
        pub port: Option<i32>,
        #[converge(strategy = converge::strategies::vec::replace_empty)]
        pub credentials: Vec<ConfigRabbitMqCredentials>,
    }

Fields may alternatively be marked with a custom strategy for a custom converge.
Where the strategy is the path to a function in the form:

    fn custom_function<T>(lhs: T, rhs: T) -> T

Where `T` matches the type of the field. The `converge` crate includes some
common generic strategies sorted in modules by container type.

## How to integrate data sources

This library is expected to be used in combination with other libraries to parse
configuration file formats, the command line, and the execution environment
variables. In practice the resultant structure presented by these libraries is
often closely bound to the input source. The `Converge trait` requires
that each source provides a common data structure. By implementing the
[From trait](https://doc.rust-lang.org/std/convert/trait.From.html) or alternatively the [TryFrom trait](https://doc.rust-lang.org/std/convert/trait.TryFrom.html) for your common data format
`Converge` can be applied to these data sources.

## Associated libraries

A non exhaustive list of data source libraries we can recommend to provide
configuration data.

* To parse the application command line:
  * [clap](https://crates.io/crates/clap)
  * [StructOpt](https://crates.io/crates/structopt)
* To parse configuration files:
  * [toml](https://crates.io/crates/toml)
  * [serde_yaml](https://crates.io/crates/serde_yaml)
  * [serde_json](https://crates.io/crates/serde_json)
  * [serde_xml_rs](https://crates.io/crates/serde-xml-rs)
* To parse the environment variables:
  * [envy](https://crates.io/crates/envy)
  * [envconfig](https://crates.io/crates/envconfig)

## Alternatives

A non exhaustive list of possible alternatives.

* [merge](https://crates.io/crates/merge)
  * Similar idea to converge.
* [twelf](https://crates.io/crates/twelf)
  * Configuration solution for Rust including 12-Factor support.
