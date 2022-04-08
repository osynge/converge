#[cfg(test)]
mod tests {
    use clone_or::CloneOr;
    extern crate pretty_assertions;
    fn config_precedence<T>(cli_cfg: T, env_cfg: T, file_cfg: T) -> T
    where
        T: CloneOr,
    {
        cli_cfg.clone_or(env_cfg).clone_or(file_cfg)
    }
    #[derive(clone_or, PartialEq, Clone)]
    struct Simple {
        number: Option<i32>,
    }
    #[test]
    fn test_patterns() {
        let empty = Simple { number: None };
        let some_1 = Simple { number: Some(1) };
        let some_2 = Simple { number: Some(2) };
        let out = config_precedence(empty.clone(), some_1.clone(), some_2.clone());
        assert!(out == some_1);
        let out = config_precedence(empty.clone(), some_2.clone(), some_1.clone());
        assert!(out == some_2);
    }
}
