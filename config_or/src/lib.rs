/*!
<<<<<<< HEAD:clone_or/src/lib.rs
# clone_or
This crate facilitates defaulting values in a simple way as a hybrid of
[Clone](std::clone::Clone) and [Option's or method](`core::option::Option::or`).
The traits defined in this crate can be derived using [clone_or].
Both [clone_or](crate) and [clone_or_derive] where created to separate order of
=======
# config_or
This crate facilitates defaulting values in a similar way to
[Option's or method](`core::option::Option::or`) but for each field in a
structure. The traits defined in this crate can be derived using [config_or_derive].
Both [config_or](crate) and [config_or_derive] where created to separate order of
>>>>>>> dfaab8b (do renaming):config_or/src/lib.rs
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


    use config_or::{ConfigOr,config_or};

    #[derive(config_or)]
    struct Simple {
        number: Option<i32>,
    }

*/
pub use config_or_derive::config_or;
pub trait ConfigOr<Rhs = Self> {
    /*! # The ConfigOr Trait.

    Structures can be coupled together to derive new instances with clear and
    simple prescience. ConfigOr can be implemented for any type. The type of
    `default` is defined as the same as self and Self.

    # Example
    ```
    use config_or::ConfigOr;

    fn config_precedence<T>(cli_cfg: T, env_cfg: T, file_cfg: T) -> T
    where
        T: ConfigOr,
    {
        cli_cfg.config_or(env_cfg).config_or(file_cfg)
    }
    ```
    */

    /// Creates Self, from all fields from self, unless the field is of type
    /// [`Option<T>`](`core::option::Option`), implements ConfigOr, or is of type
    /// [`Option<T>`](`core::option::Option`) where T implements ConfigOr.
    ///
    /// If the field of self:
    ///  * is of type [`Option<T>`](`core::option::Option`), and has the
    /// value [None](`core::option::Option::None`), the fields value is cloned
    /// from default's field instead.
    /// * implements ConfigOr, then the result of calling config_or on that field,
    /// with the parameter from defaults field.
    /// * is of type [`Option<T>`](`core::option::Option`), where T
    /// implements the [ConfigOr trait](Self) and the value is
    /// [`Some<T>`](`core::option::Option::Some`) for both
    /// self and default, the returned structures fields value is
    /// [`Some(val)`](`core::option::Option::Some`) where `val` is the result of
    /// calling config_or on the field's value wrapped by
    /// [`Some`](`core::option::Option::Some`) for both self and default.

    fn config_or(self, default: Rhs) -> Self;
}
/** If the both `self` option, and `default` option is a Some value, then return
 Some using config_or method. If the both `self` option, and `default` option
 are None, then return None. If `self` option, or `default` option have Some
 value return Some with that value.
*/
impl<T> ConfigOr for Option<T>
where
    T: ConfigOr,
{
    /** If the both `self` option, and `default` option is a Some value, then return
     Some using config_or method. If the both `self` option, and `default` option
     are None, then return None. If `self` option, or `default` option have Some
     value return Some with that value.
    */
    fn config_or(self, default: Option<T>) -> Self {
        match (self, default) {
            (Some(ps), Some(pd)) => Some(ps.config_or(pd)),
            (Some(ps), None) => Some(ps),
            (None, Some(pd)) => Some(pd),
            (None, None) => None,
        }
    }
}
