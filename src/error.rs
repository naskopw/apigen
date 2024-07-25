use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("json error: {0}")]
    Json(#[from] serde_json::Error),

    #[error("spec error: {0}")]
    Spec(#[from] oas3::Error),

    #[error("io error: {0}")]
    Io(#[from] std::io::Error),

    #[error("plugin not found: {0}")]
    #[allow(dead_code)]
    PluginNotFound(String),

    #[error("plugin execution error: {0}")]
    #[allow(dead_code)]
    PluginExec(std::io::Error),
}
