use crate::mergeable;
use std::convert::TryFrom;

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

fn required_variables_extract(
    item: &mergeable::Config,
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

impl TryFrom<mergeable::Config> for Config {
    type Error = ConfigError;

    fn try_from(item: mergeable::Config) -> Result<Self, Self::Error> {
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
    mergeable::get_config(args, envs)?.try_into()
}
