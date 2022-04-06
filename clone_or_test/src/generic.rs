#[cfg(test)]

mod tests {
    use clone_or::CloneOr;
    extern crate pretty_assertions;
    fn config_precedence<T>(cli_cfg: &T, env_cfg: &T, file_cfg: &T) -> T
    where
        T: CloneOr,
    {
        cli_cfg
            .clone_or(env_cfg)
            .clone_or(env_cfg)
            .clone_or(file_cfg)
    }
    #[derive(clone_or, PartialEq)]
    struct Simple {
        number: Option<i32>,
    }
    #[test]
    fn test_patterns() {
        let empty = Simple { number: None };
        let some_1 = Simple { number: Some(1) };
        let some_2 = Simple { number: Some(2) };
        let out = config_precedence(&empty, &some_1, &some_2);
        assert!(out == some_1);
        let out = config_precedence(&empty, &some_2, &some_1);
        assert!(out == some_2);
    }
}
