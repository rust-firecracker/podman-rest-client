#[derive(thiserror::Error, Debug)]
pub enum ClientError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    #[error("Invalid URI: {0}")]
    InvalidUri(#[from] http::uri::InvalidUri),
    #[error("SSH Authentication Failed")]
    AuthenticationFailed,
    #[error("Missing or unsupported scheme in URI")]
    InvalidScheme,
}
