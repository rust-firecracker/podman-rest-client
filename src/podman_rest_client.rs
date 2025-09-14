use hyper_util::client::legacy::Client;
use hyper_util::rt::TokioExecutor;

use crate::api_common::config::HasConfig;
use crate::api_common::ClientConfig;
use crate::api_common::Config as APIConfig;
use crate::api_common::Connector;
use crate::config::Config;
use crate::error::ClientError;
use crate::impl_crate_v5_traits;
use crate::unix_socket;

const BASE_PATH: &str = "http://d/v5.1.0";

pub struct PodmanRestClient {
    config: Box<dyn ClientConfig>,
}

impl_crate_v5_traits!(PodmanRestClient);

impl HasConfig for PodmanRestClient {
    fn get_config(&self) -> &dyn ClientConfig {
        &*self.config
    }
}

impl PodmanRestClient {
    pub async fn new(config: Config) -> Result<Self, ClientError> {
        let (scheme, rest) = config
            .uri
            .split_once("://")
            .ok_or(ClientError::InvalidScheme)?;

        match scheme {
            "unix" => Ok(PodmanRestClient::new_unix(rest)),
            _ => Err(ClientError::InvalidScheme),
        }
    }

    pub fn new_unix(path: &str) -> PodmanRestClient {
        let connector = unix_socket::UnixConnector::new(path);

        PodmanRestClient::new_connector(connector)
    }

    fn new_connector<C: Connector>(connector: C) -> PodmanRestClient {
        let client = Client::builder(TokioExecutor::new()).build(connector);

        PodmanRestClient {
            config: Box::new(APIConfig {
                base_path: BASE_PATH.to_string(),
                ..APIConfig::with_client(client)
            }),
        }
    }

    pub fn v5(&self) -> &dyn crate::v5::Client {
        self
    }
}
