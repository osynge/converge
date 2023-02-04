use clap::Parser;
use converge::Converge;
use serde::Deserialize;
use std::convert::TryFrom;

#[derive(Debug)]
pub struct Config {
    // Toml config file
    pub config_file: Option<std::path::PathBuf>,

    /// Rest API network address
    pub host: std::net::Ipv4Addr,

    /// Rest API network port
    pub port: u16,

    // Postgres Server address
    pub pg_host: std::net::Ipv4Addr,

    // Postgres Server port
    pub pg_port: u16,

    // Postgres DB name
    pub pg_db: Option<String>,

    // Postgres user
    pub pg_user: String,

    // Postgres user
    pub pg_password: String,
}

/// Config from Sources
#[derive(Parser, Debug, Deserialize)]
#[clap(author, version, about, long_about = None)]
#[derive(Converge)]
pub(super) struct ConfigToConverge {
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

impl ConfigToConverge {
    pub fn default() -> ConfigToConverge {
        ConfigToConverge {
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

#[derive(Debug, thiserror::Error)]
pub enum ConfigError {
    #[error("errors: {errors:?}")]
    MissingField { errors: Vec<&'static str> },
    #[error(transparent)]
    EnvConfig(#[from] envy::Error),
    #[error(transparent)]
    TomlConfig(#[from] toml::de::Error),
    #[error(transparent)]
    IoConfig(#[from] std::io::Error),
}

fn required_variables_extract(
    item: &ConfigToConverge,
) -> Result<
    (
        std::net::Ipv4Addr,
        u16,
        std::net::Ipv4Addr,
        u16,
        String,
        String,
    ),
    ConfigError,
> {
    if let (
        Some(host),
        Some(port),
        Some(pg_host),
        Some(pg_port),
        Some(pg_user),
        Some(pg_password),
    ) = (
        item.host,
        item.port,
        item.pg_host,
        item.pg_port,
        item.pg_user.clone(),
        item.pg_password.clone(),
    ) {
        return Ok((host, port, pg_host, pg_port, pg_user, pg_password));
    }
    let mut errors = Vec::new();
    if item.host.is_none() {
        errors.push("Rest API 'address' is undefined.")
    }
    if item.port.is_none() {
        errors.push("Rest API 'port' is undefined.")
    }
    if item.pg_host.is_none() {
        errors.push("Rest API 'pg_host' is undefined.")
    }
    if item.pg_port.is_none() {
        errors.push("Rest API 'pg_port' is undefined.")
    }
    if item.pg_user.is_none() {
        errors.push("Rest API 'pg_user' is undefined.")
    }
    if item.pg_password.is_none() {
        errors.push("Rest API 'pg_password' is undefined.")
    }
    Err(ConfigError::MissingField { errors })
}

impl TryFrom<ConfigToConverge> for Config {
    type Error = ConfigError;

    fn try_from(item: ConfigToConverge) -> Result<Self, Self::Error> {
        let (host, port, pg_host, pg_port, pg_user, pg_password) =
            required_variables_extract(&item)?;
        Ok(Config {
            config_file: item.config_file,
            host,
            port,
            pg_host,
            pg_port,
            pg_db: item.pg_db,
            pg_user,
            pg_password,
        })
    }
}

pub fn get_config<I, J, T>(args: I, envs: J) -> Result<Config, ConfigError>
where
    I: IntoIterator<Item = T>,
    T: Into<std::ffi::OsString> + Clone,
    J: Iterator<Item = (std::ffi::OsString, std::ffi::OsString)>,
{
    // Make a config collector with default values.
    let mut config_collector = ConfigToConverge::default();
    let local_iter = envs.filter_map(|(key, val)| match (key.into_string(), val.into_string()) {
        (Ok(new_key), Ok(new_val)) => Some((new_key, new_val)),
        (_, _) => None,
    });
    let env = envy::prefixed("CONVERGE_EXAMPLE_CONCISE_").from_iter(local_iter)?;
    let cli = ConfigToConverge::parse_from(args);
    // Get the converged cli and env config.
    let cli_env = cli.converge(env);
    // converge cfg with config collector.
    if let Some(file_path) = &cli_env.config_file {
        let content = std::fs::read_to_string(file_path)?;
        config_collector =
            toml::from_str::<ConfigToConverge>(content.as_str())?.converge(config_collector);
    };
    // converge cli env cfg and defaults.
    cli_env.converge(config_collector).try_into()
}
