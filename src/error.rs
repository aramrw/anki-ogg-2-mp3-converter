use thiserror::Error;

#[derive(Error, Debug)]
pub enum ConfigError {
    //#[error("missing config key: {0}")]
    //MissingKey(String),
    #[error("io err: {0}")]
    Io(#[from] std::io::Error),
    #[error("parsing err: {0}")]
    Json(#[from] serde_json::Error),
    #[error("config does not exist")]
    MissingConfig()
}
