/*!
# clone_or
This crate facilitates defaulting values in a simple way as a hybrid of
[Clone](std::clone::Clone) and [Option's or method](`core::option::Option::or`).
The traits defined in this crate can be derived using [clone_or_derive].
Both [clone_or](crate) and [clone_or_derive] where created to separate order of
precedence from configuration content.

## Motivation

Both the [Unix philosophy](https://en.wikipedia.org/wiki/Unix_philosophy) and
the idea of [micro services](https://en.wikipedia.org/wiki/Microservices) lead
to writing lots of applications with configuration
[boiler plate code](https://en.wikipedia.org/wiki/Boilerplate_code).

It is reasonable to expect these applications will take configuration from the
command line arguments, environment and one or more configuration files. It is
also expected that the more specific configuration files will override the more
general. The environment configuration overrides the configuration files,
and the command line arguments overrides the environment and file based
configuration. Also the command line and environment configuration, may specify
the location of a configuration file, that overrides all the other configuration
files.

As applications mature the number of configuration options also tends to
grow adding to the complexity. Testing of configuration overriding for each
configuration option further increases the quantity of
[boiler plate code](https://en.wikipedia.org/wiki/Boilerplate_code).

This crate keeps the configuration override precedence separate from
configuration content reducing the quantity and complexity of the code base, and
with it the amount of testing needed.


    use clone_or_derive::clone_or;
    use clone_or::CloneOr;

    #[derive(clone_or)]
    struct Simple {
        number: Option<i32>,
    }

*/

pub trait CloneOr<Rhs = Self> {
    /*! # The CloneOr Trait.

    Structures can be coupled together to derive new instances with clear and
    simple prescience. CloneOr can be implemented for any type. The type of
    `default` is defined as the same as self and Self.

    # Example
    ```
    use clone_or;

    fn config_precedence<T>(cli_cfg: &T, env_cfg: &T, file_cfg: &T) -> T
    where
        T: clone_or::CloneOr,
    {
        cli_cfg
            .clone_or(env_cfg)
            .clone_or(env_cfg)
            .clone_or(file_cfg)
    }
    ```
    */

    /// Clones self, and all fields from self, unless the field is of type
    /// [`Option<T>`](`core::option::Option`), implements CloneOr, or is of type
    /// [`Option<T>`](`core::option::Option`) where T implements CloneOr.
    ///
    /// If the field of self:
    ///  * is of type [`Option<T>`](`core::option::Option`), and has the
    /// value [None](`core::option::Option::None`), the fields value is cloned
    /// from default's field instead.
    /// * implements CloneOr, then the result of calling clone_or on that field,
    /// with the parameter from defaults field.
    /// * is of type [`Option<T>`](`core::option::Option`), where T
    /// implements the [CloneOr trait](Self) and the value is
    /// [`Some<T>`](`core::option::Option::Some`) for both
    /// self and default, the returned structures fields value is
    /// [`Some(val)`](`core::option::Option::Some`) where `val` is the result of
    /// calling clone_or on the field's value wrapped by
    /// [`Some`](`core::option::Option::Some`) for both self and default.

    fn clone_or(&self, default: &Rhs) -> Self;
}
