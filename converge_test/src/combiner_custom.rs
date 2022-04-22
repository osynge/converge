#[cfg(test)]

mod tests {
    use converge::Converge;
    extern crate pretty_assertions;

    #[derive(Converge, PartialEq)]
    struct Simple {
        #[converge(strategy = converge::strategies::vec::replace_empty)]
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

    #[derive(Converge, PartialEq)]
    struct Complex {
        #[converge(strategy = converge::strategies::vec::converge_union)]
        simple: Vec<Simple>,
    }
    impl Complex {
        pub fn new_with(value: &[&[i16]]) -> Complex {
            Complex {
                simple: value.iter().map(|x| Simple::new_with(x)).collect(),
            }
        }
    }

    #[test]
    fn test_simple_self_some_default_some() {
        let some_1 = Simple::new_with(&[1]);
        let some_2 = Simple::new_with(&[2]);
        let out = some_1.converge(some_2);
        assert!(out == Simple::new_with(&[1]));
    }

    #[test]
    fn test_simple_self_none_default_some() {
        let empty = Simple::new_empty();
        let some_2 = Simple::new_with(&[2]);
        let out = empty.converge(some_2);
        assert!(out == Simple::new_with(&[2]));
    }

    #[test]
    fn test_simple_self_empty_default_some() {
        let empty = Simple::new_with(&[]);
        let some_2 = Simple::new_with(&[2]);
        let out = empty.converge(some_2);
        assert!(out == Simple::new_with(&[2]));
    }

    #[test]
    fn test_simple_self_none_default_none() {
        let empty_1 = Simple::new_empty();
        let empty_2 = Simple::new_empty();
        let out = empty_1.converge(empty_2);
        assert!(out == Simple::new_empty());
    }

    #[test]
    fn test_simple_self_some_default_none() {
        let some_1 = Simple::new_with(&[1]);
        let empty = Simple::new_empty();
        let out = some_1.converge(empty);
        assert!(out == Simple::new_with(&[1]));
    }
    #[test]
    fn test_complex_self_some_default_some() {
        let some_1 = Complex::new_with(&[&[1], &[], &[4]]);
        let some_2 = Complex::new_with(&[&[2], &[3]]);
        let out = some_1.converge(some_2);
        assert!(out == Complex::new_with(&[&[1], &[3], &[4]]));
    }
}
