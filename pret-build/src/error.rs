use std::io;

use toml;

pub type TResult<T> = core::result::Result<T, Error>;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("{0}")]
    IoError(#[from] io::Error),

    #[error("Unable to deserialize Pret Spec File: {0}")]
    DeserializationError(#[from] toml::de::Error),

    #[error("Unable to deserialize Cargo Manifest: {0}")]
    ManifestError(#[from] cargo_metadata::Error),

    #[error("Dependency resolution failed: {0}")]
    DependencyError(String)
}