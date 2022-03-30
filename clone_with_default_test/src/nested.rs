#[cfg(test)]

mod tests {
    use clone_with_default::CloneWithDefault;
    extern crate pretty_assertions;

    #[derive(Clone, CloneWithDefault)]
    struct FrenchToast {
        frog: Option<i32>,
        jam: Option<i32>,
    }

    #[derive(Clone, CloneWithDefault)]
    struct Waffles {
        #[cwd]
        ding: FrenchToast,
        frog: Option<i32>,
        jam: Option<i32>,
        #[cwd]
        elephant: Option<FrenchToast>,
        doowapp: Option<Vec<String>>,
    }

    #[test]
    fn test_add() {
        let j = FrenchToast {
            frog: Some(1),
            jam: None,
        };
        let d = j.clone_with_default(&j);
        let _k = Waffles {
            ding: d.clone(),
            frog: Some(1),
            jam: None,
            elephant: Some(d.clone()),
            doowapp: Some(vec![]),
        };
    }
}
