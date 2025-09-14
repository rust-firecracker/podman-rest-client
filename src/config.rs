use crate::cli;

pub struct Config {
    pub uri: String,
    pub identity_file: Option<String>,
}

#[derive(thiserror::Error, Debug)]
pub enum GuessError {
    #[error("No default podman connection info found")]
    NoDefault,
    #[error("Cli error: {0}")]
    Cli(#[from] cli::CliError),
}

impl Config {
    pub async fn guess() -> Result<Config, GuessError> {
        for path_socket in [
            &format!("/run/user/{}/podman/podman.sock", nix::unistd::getuid()),
            "/run/podman/podman.sock",
        ] {
            if std::path::Path::new(path_socket).exists() {
                return Ok(Config {
                    uri: format!("unix://{path_socket}"),
                    identity_file: None,
                });
            }
        }

        Err(GuessError::NoDefault)
    }
}
