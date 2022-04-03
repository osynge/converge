#[cfg(test)]

mod tests {
    use clone_with_default::CloneWithDefault;
    extern crate pretty_assertions;

    #[derive(Copy, Clone, PartialEq)]
    enum EarType {
        Lop,
        Erect,
    }
    #[derive(Copy, Clone, PartialEq)]
    enum FurType {
        Long,
        Short,
        Medium,
    }

    #[derive(Copy, Clone, PartialEq)]
    enum Color {
        White,
        Blue,
        Red,
        Ivory,
        Sandy,
        Brown,
    }

    #[derive(Copy, Clone, CloneWithDefault, PartialEq)]
    struct Ears {
        fur_type: Option<FurType>,
        ear_type: Option<EarType>,
    }

    #[derive(Copy, Clone, CloneWithDefault, PartialEq)]
    struct Rabbit {
        fur_type: Option<FurType>,
        color: Option<Color>,
        #[cwd]
        ears: Option<Ears>,
    }

    impl Ears {
        pub fn new() -> Ears {
            Ears {
                fur_type: None,
                ear_type: None,
            }
        }
    }
    impl Rabbit {
        pub fn new() -> Rabbit {
            Rabbit {
                fur_type: None,
                color: None,
                ears: None,
            }
        }
    }
    fn gen_cashmere_lop() -> Rabbit {
        Rabbit {
            fur_type: Some(FurType::Short),
            color: Some(Color::Sandy),
            ears: Some(Ears {
                fur_type: Some(FurType::Short),
                ear_type: Some(EarType::Lop),
            }),
        }
    }
    fn gen_dutch() -> Rabbit {
        Rabbit {
            fur_type: Some(FurType::Short),
            color: Some(Color::Brown),
            ears: Some(Ears {
                fur_type: Some(FurType::Short),
                ear_type: Some(EarType::Erect),
            }),
        }
    }

    #[test]
    fn test_can_clone() {
        let undefined = Rabbit::new();
        let george = gen_cashmere_lop();
        let baby = george.clone_with_default(&undefined);
        assert!(baby == george)
    }
    #[test]
    fn test_can_default() {
        let emma = Rabbit::new();
        let george = gen_cashmere_lop();
        let baby = emma.clone_with_default(&george);
        assert!(baby == george)
    }

    #[test]
    fn test_can_ignore() {
        let emma = gen_dutch();
        let george = gen_cashmere_lop();
        let baby = emma.clone_with_default(&george);
        assert!(baby == emma)
    }

    #[test]
    fn test_can_hybrid() {
        let emma = gen_dutch();
        let mut george = gen_cashmere_lop();
        george.ears = Some(Ears {
            ear_type: None,
            fur_type: Some(FurType::Long),
        });
        let baby = george.clone_with_default(&emma);
        assert!(george.fur_type == baby.fur_type);
        assert!(george.color == baby.color);
        assert!(baby.ears.unwrap().ear_type == emma.ears.unwrap().ear_type);
        assert!(baby.ears.unwrap().fur_type == george.ears.unwrap().fur_type);
    }
}
