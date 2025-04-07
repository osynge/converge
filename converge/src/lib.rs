/*!
# converge
This crate facilitates defaulting values in a similar way to
[Option's or method](`core::option::Option::or`) but for each field in a
structure. The traits defined in this crate can be derived using [converge_derive].
Both [converge](crate) and [converge_derive] where created to separate order of
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


    use converge::Converge;

    #[derive(Converge)]
    struct Simple {
        number: Option<i32>,
    }

*/
pub use converge_derive::Converge;
pub mod strategies;
pub trait Converge<Rhs = Self> {
    /*! # The Converge Trait.

    Structures can be coupled together to derive new instances with clear and
    simple prescience. Converge can be implemented for any type. The type of
    `default` is defined as the same as self and Self.

    # Example
    ```
    use converge::Converge;

    fn config_precedence<T>(cli_cfg: T, env_cfg: T, file_cfg: T) -> T
    where
        T: Converge,
    {
        cli_cfg.converge(env_cfg).converge(file_cfg)
    }
    ```
    */

    /// Creates Self, from all fields from self, unless the field is of type
    /// [`Option<T>`](`core::option::Option`), implements Converge, or is of type
    /// [`Option<T>`](`core::option::Option`) where T implements Converge.
    ///
    /// If the field of self:
    ///  * is of type [`Option<T>`](`core::option::Option`), and has the
    /// value [None](`core::option::Option::None`), the fields value is cloned
    /// from default's field instead.
    /// * implements Converge, then the result of calling converge on that field,
    /// with the parameter from defaults field.
    /// * is of type [`Option<T>`](`core::option::Option`), where T
    /// implements the [Converge trait](Self) and the value is
    /// [`Some<T>`](`core::option::Option::Some`) for both
    /// self and default, the returned structures fields value is
    /// [`Some(val)`](`core::option::Option::Some`) where `val` is the result of
    /// calling converge on the field's value wrapped by
    /// [`Some`](`core::option::Option::Some`) for both self and default.
    fn converge(self, default: Rhs) -> Self;
}
/** Implementing Converge for Option of a structure implementing Converge
simplifies the derive macro and can be of use.
*/
impl<T> Converge for Option<T>
where
    T: Converge,
{
    /** If the both `self` option, and `default` option is a Some value, then return
     Some using converge method. If the both `self` option, and `default` option
     are None, then return None. If `self` option, or `default` option have Some
     value return Some with that value.
    */
    fn converge(self, default: Option<T>) -> Self {
        match (self, default) {
            (Some(ps), Some(pd)) => Some(ps.converge(pd)),
            (Some(ps), None) => Some(ps),
            (None, Some(pd)) => Some(pd),
            (None, None) => None,
        }
    }
}
