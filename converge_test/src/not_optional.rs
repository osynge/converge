#[cfg(test)]
mod tests {
    use converge::Converge;
    extern crate pretty_assertions;

    #[derive(PartialEq)]
    struct NoClone {
        number: u32,
    }

    #[derive(Converge, PartialEq)]
    struct NotOptional {
        no_clone: NoClone,
    }

    #[test]
    fn test_not_optional() {
        let some_1 = NotOptional {
            no_clone: NoClone { number: 1 },
        };
        let some_2 = NotOptional {
            no_clone: NoClone { number: 2 },
        };
        let out = some_1.converge(some_2);
        assert!(
            out == NotOptional {
                no_clone: NoClone { number: 1 },
            }
        );
    }
}
