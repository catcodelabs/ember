use std::path::PathBuf;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum EmberError {
    #[error("Failed to read or write file: {0}")]
    Io(#[from] std::io::Error),

    #[error("Failed to parse .anvil blueprint: {0}")]
    TomlDeserialization(#[from] toml::de::Error),

    #[error("Artifact not found after build at target path: {0:?}")]
    ArtifactNotFound(PathBuf),

    #[error("Build step execution failed with exit code: {0}")]
    BuildStepFailed(i32),

    #[error("Python execution failed: {0}")]
    PythonScriptError(String),
}
