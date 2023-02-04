use thiserror::Error;

#[derive(Error, Debug)]
pub(crate) enum ConfigureErr {
    #[error("File not found '{0}'.")]
    TomlErr(#[from] toml::de::Error),
    #[error("io parsing error")]
    IoErr(#[from] std::io::Error),
    #[error("File not found '{0}'.")]
    FilePathIsNotFile(String),
}
