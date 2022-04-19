#[cfg(test)]
mod tests {
    use converge::Converge;
    extern crate pretty_assertions;

    #[derive(Converge, PartialEq, Clone)]
    struct Simple<'a, T: 'a>
    where
        T: Clone,
    {
        contained: Option<&'a T>,
    }

    impl<'a, T> Simple<'a, T>
    where
        T: Clone,
    {
        pub fn new_empty() -> Simple<'a, T> {
            Simple {
                contained: std::option::Option::None,
            }
        }
        pub fn new_with(value: &'a T) -> Simple<'a, T> {
            Simple {
                contained: std::option::Option::Some(value),
            }
        }
    }

    #[test]
    fn test_self_some_default_some() {
        let some_1 = Simple::new_with(&"1");
        let some_2 = Simple::new_with(&"2");
        let out = some_1.converge(some_2);
        assert!(out == Simple::new_with(&"1"));
    }

    #[test]
    fn test_self_none_default_some() {
        let empty = Simple::new_empty();
        let some_2 = Simple::new_with(&"2");
        let out = empty.converge(some_2);
        assert!(out == Simple::new_with(&"2"));
    }

    #[test]
    fn test_self_none_default_none() {
        let empty_1: Simple<&str> = Simple::new_empty();
        let empty_2 = Simple::new_empty();
        let out = empty_1.converge(empty_2);
        assert!(out == Simple::new_empty());
    }

    #[test]
    fn test_self_some_default_none() {
        let some_1 = Simple::new_with(&"1");
        let empty = Simple::new_empty();
        let out = some_1.converge(empty);
        assert!(out == Simple::new_with(&"1"));
    }
}
