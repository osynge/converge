#[cfg(test)]

mod tests {
    use config_or::ConfigOr;
    extern crate pretty_assertions;

    #[derive(ConfigOr, PartialEq)]
    struct Simple {
        #[combine(combiner = simple_custom_combiner)]
        number: Vec<i16>,
    }

    impl Simple {
        pub fn new_empty() -> Simple {
            Simple { number: Vec::new() }
        }
        pub fn new_with(value: &[i16]) -> Simple {
            Simple {
                number: Vec::from(value),
            }
        }
    }

    fn simple_custom_combiner(a: Vec<i16>, b: Vec<i16>) -> Vec<i16> {
        match a.len() == 0 {
            false => a,
            true => b,
        }
    }

    #[test]
    fn test_self_some_default_some() {
        let some_1 = Simple::new_with(&[1]);
        let some_2 = Simple::new_with(&[2]);
        let out = some_1.config_or(some_2);
        assert!(out == Simple::new_with(&[1]));
    }

    #[test]
    fn test_self_none_default_some() {
        let empty = Simple::new_empty();
        let some_2 = Simple::new_with(&[2]);
        let out = empty.config_or(some_2);
        assert!(out == Simple::new_with(&[2]));
    }

    #[test]
    fn test_self_none_default_none() {
        let empty_1 = Simple::new_empty();
        let empty_2 = Simple::new_empty();
        let out = empty_1.config_or(empty_2);
        assert!(out == Simple::new_empty());
    }

    #[test]
    fn test_self_some_default_none() {
        let some_1 = Simple::new_with(&[1]);
        let empty = Simple::new_empty();
        let out = some_1.config_or(empty);
        assert!(out == Simple::new_with(&[1]));
    }
}
