pub fn cli_env() -> crate::Config {
    let mut out = crate::Config::default();
    for (key, value) in std::env::vars() {
        if "XRC_CONFIG".eq(&key) {
            out.config_file = Some(value.clone());
        }
        if "XRC_XUNIT".eq(&key) {
            out.xunit_local_globs = Some(
                value
                    .clone()
                    .split(':')
                    .into_iter()
                    .map(String::from)
                    .collect(),
            );
        }
        if "XRC_ENVIROMENT_KEY".eq(&key) {
            out.environment_sk = Some(value.clone());
        }
        if "XRC_ENVIROMENT".eq(&key) {
            out.environment_keys = Some(
                value
                    .clone()
                    .split(':')
                    .into_iter()
                    .map(String::from)
                    .collect(),
            );
        }
        if "XRC_PROJECT_KEY".eq(&key) {
            out.project_sk = Some(value.clone());
        }
        if "XRC_PROJECT_IDENTIFIER".eq(&key) {
            out.project_identifier = Some(value.clone());
        }
        if "XRC_PROJECT_NAME".eq(&key) {
            out.project_human_name = Some(value.clone());
        }
        if "XRC_RUN_IDENTIFIER".eq(&key) {
            out.run_identifier = Some(value.clone());
        }
        if "XRC_RUN_KEY".eq(&key) {
            out.run_sk = Some(value.clone());
        }
        if "XRC_SERVICE_URL".eq(&key) {
            out.service_url = Some(value.clone());
        }
    }
    out
}
