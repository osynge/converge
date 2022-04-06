mod tests {
    use clone_or::CloneOr;

    use clone_or_derive::clone_or;

    extern crate pretty_assertions;

    #[derive(Debug, PartialEq, clone_or)]
    pub struct Config {
        pub config_file: Option<String>,
        pub loglevel: Option<i8>,
        pub xunit_local_globs: Option<Vec<String>>,
        pub environment_sk: Option<String>,
        pub environment_keys: Option<Vec<String>>,
        pub project_sk: Option<String>,
        pub project_identifier: Option<String>,
        pub project_human_name: Option<String>,
        pub run_identifier: Option<String>,
        pub run_sk: Option<String>,
        pub service_url: Option<String>,
    }
    impl Config {
        //set the method's context
        pub fn new() -> Config {
            Config {
                config_file: None,
                loglevel: None,
                xunit_local_globs: None,
                environment_sk: None,
                environment_keys: None,
                project_sk: None,
                project_identifier: None,
                project_human_name: None,
                run_identifier: None,
                run_sk: None,
                service_url: None,
            }
        }
    }
    fn gen_config_with_data_1() -> Config {
        Config {
            config_file: Some(String::from("configfile")),
            loglevel: Some(1),
            xunit_local_globs: Some(vec![String::from("xunit_local_globs")]),
            environment_sk: Some(String::from("environment_sk")),
            environment_keys: Some(vec![String::from("environment_keys")]),
            project_sk: Some(String::from("project_sk")),
            project_identifier: Some(String::from("project_identifier")),
            project_human_name: Some(String::from("project_human_name")),
            run_identifier: Some(String::from("run_identifier")),
            run_sk: Some(String::from("run_sk")),
            service_url: Some(String::from("service_url")),
        }
    }
    fn gen_config_with_data_2() -> Config {
        Config {
            config_file: Some(String::from("2")),
            loglevel: Some(1),
            xunit_local_globs: Some(vec![String::from("2")]),
            environment_sk: Some(String::from("2")),
            environment_keys: Some(vec![String::from("2")]),
            project_sk: Some(String::from("2")),
            project_identifier: Some(String::from("2")),
            project_human_name: Some(String::from("2")),
            run_identifier: Some(String::from("2")),
            run_sk: Some(String::from("2")),
            service_url: Some(String::from("2")),
        }
    }

    #[test]
    fn gets_default_with_none() {
        let a = gen_config_with_data_1();
        let b = Config::new();
        let c = b.clone_or(&a);
        assert_eq!(c, a);
    }

    #[test]
    fn gets_none_with_none() {
        let a = Config::new();
        let b = Config::new();
        let c = b.clone_or(&a);
        assert_eq!(c, a);
    }

    #[test]
    fn gets_original_with_none() {
        let a = gen_config_with_data_1();
        let b = Config::new();
        let c = a.clone_or(&b);
        assert_eq!(c, a);
    }

    #[test]
    fn gets_original_with_some() {
        let a = gen_config_with_data_1();
        let b = gen_config_with_data_2();
        let c = a.clone_or(&b);
        assert_eq!(c, a);
    }
}
