# CloneWithDefault

A Layered configuration system for Rust applications.

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

It is based on a trait `CloneWithDefault` with a single method
`clone_with_default` this is in the crate `clone_with_default`.

    pub trait CloneWithDefault<Rhs = Self> {
        fn clone_with_default(&self, default: &Rhs) -> Self;
    }

This trait can be derived using `clone_with_default_derive` as shown below.

    #[derive(CloneWithDefault)]
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
    let cfg = config_commandline.clone_with_default(&config_file).clone_with_default(config_env);

## Designing your configuration structure

The structure implementing the trait `CloneWithDefault` should contain a logical
grouping of fields to represent each setting you may wish to use as part of your
layered configuration.

When deriving `CloneWithDefault` with `clone_with_default_derive` you can mark
fields as also supporting `CloneWithDefault`, this then allows

### Example deriving `CloneWithDefault` with nested structures

Nested

  #[derive(CloneWithDefault)]
    pub struct ConfigRabbitMqCredentials {
        pub username: Option<String>,
        pub password: Option<String>,
    }

    #[derive(CloneWithDefault)]
    pub struct ConfigRabbitMQ {
        pub host: Option<String>,
        pub port: Option<i32>,
        pub credentials: Option<ConfigRabbitMqCredentials>,
    }

When using Fields that also implement `CloneWithDefault` shoudl

As can be seen in the Quickstart These fields are typically
[Option](https://doc.rust-lang.org/std/option/)\<T\>. This allows the
implementation to detect that the field is not set and get it from the default
should that have a value. It is possible to have T typed fields that are not
Optional values, but this not usually as it reduces the

The type T is bound to implement `CloneWithDefault` or `Clone`.

## How to integrate data sources

This library is expected to be used in combination with other libraries to parse
configuration file formats, the command line, and the execution environment
variables. In practice the resultant structure presented by these libraries is
often closely bound to the input source. The `CloneWithDefault trait` requires
that each source provides a common data structure. By implementing the
[From trait](https://doc.rust-lang.org/std/convert/trait.From.html) or
alternatively the
[TryFrom trait](https://doc.rust-lang.org/std/convert/trait.TryFrom.html) for
your common data format `CloneWithDefault` can be applied to these data sources.

As a non exhaustive list of data source libraries we can recommend:

* To parse the application command line:
  * clap
  * StructOpt
* To parse configuration files:
  * toml
  * serde_xml_rs
  * serde_yaml
  * serde_json
