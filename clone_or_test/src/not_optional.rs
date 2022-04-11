#[cfg(test)]
mod tests {
    use clone_or::{clone_or, CloneOr};
    extern crate pretty_assertions;

    #[derive(PartialEq)]
    struct NoClone {
        number: u32,
    }

    #[derive(clone_or, PartialEq)]
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
        let out = some_1.clone_or(some_2);
        assert!(
            out == NotOptional {
                no_clone: NoClone { number: 1 },
            }
        );
    }
}
