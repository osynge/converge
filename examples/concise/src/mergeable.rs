use clap::Parser;
use converge::Converge;
use serde::Deserialize;

/// Core10 persistence.
#[derive(Parser, Debug, Deserialize)]
#[clap(author, version, about, long_about = None)]
#[derive(Converge)]
pub(super) struct Config {
    // Toml config file
    #[clap(long, action)]
    pub config_file: Option<std::path::PathBuf>,

    /// Rest API network port
    #[clap(long, action)]
    #[serde(rename = "port")]
    pub port: Option<u16>,

    /// Rest API network address
    #[clap(long, action)]
    #[serde(rename = "host")]
    pub host: Option<std::net::Ipv4Addr>,

    // Postgres Server address
    #[clap(long, action)]
    #[serde(rename = "pg_host")]
    pub pg_host: Option<std::net::Ipv4Addr>,

    // Postgres Server port
    #[clap(long, action)]
    #[serde(rename = "pg_port")]
    pub pg_port: Option<u16>,

    // Postgres DB name
    #[clap(long, action)]
    #[serde(rename = "pg_db")]
    pub pg_db: Option<String>,

    // Postgres user
    #[clap(long, action)]
    #[serde(rename = "pg_user")]
    pub pg_user: Option<String>,

    // Postgres user
    #[clap(long, action)]
    #[serde(rename = "pg_password")]
    pub pg_password: Option<String>,
}

impl Config {
    pub fn default() -> Config {
        Config {
            config_file: None,
            port: None,
            host: None,
            pg_host: None,
            pg_port: Some(5432),
            pg_db: None,
            pg_user: None,
            pg_password: None,
        }
    }
}

pub(super) fn get_config<I, J, T>(args: I, envs: J) -> Result<Config, crate::config::ConfigError>
where
    I: IntoIterator<Item = T>,
    T: Into<std::ffi::OsString> + Clone,
    J: Iterator<Item = (std::ffi::OsString, std::ffi::OsString)>,
{
    // Make a config collector with default values.
    let mut config_collector = Config::default();
    let local_iter = envs.filter_map(|(key, val)| match (key.into_string(), val.into_string()) {
        (Ok(new_key), Ok(new_val)) => Some((new_key, new_val)),
        (_, _) => None,
    });
    let env = envy::prefixed("CORE10_").from_iter(local_iter)?;
    let cli = Config::parse_from(args);
    // Get the convereged cli and env config.
    let cli_env = cli.converge(env);
    // converge cfg with config collector.
    if let Some(file_path) = &cli_env.config_file {
        let content = std::fs::read_to_string(file_path)?;
        config_collector = toml::from_str::<Config>(content.as_str())?.converge(config_collector);
    };
    // converge cli env cfg and defaults.
    Ok(cli_env.converge(config_collector))
}
