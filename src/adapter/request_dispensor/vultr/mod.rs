use crate::errors::ServiceError;
use interfaces::ExecuteVultrCommand;
use reqwest::{Client, Method, RequestBuilder};
use schemas::BASE_URL;
use serde::Deserialize;
use std::sync::OnceLock;

use super::get_client;

pub mod interfaces;
pub mod schemas;

pub struct VultrClient {
    client: &'static Client,
    api_key: String,
}

impl VultrClient {
    fn new(api_key: String) -> Self {
        Self {
            client: get_client(),
            api_key,
        }
    }

    pub(crate) fn build_request(&self, method: Method, url: String) -> RequestBuilder {
        // let prefix = if cfg!(test) { MOCKING_SERVER_URL } else { BASE_URL };
        let prefix = BASE_URL;
        let url = format!("{}/{}", prefix, url);
        self.client
            .request(method, url)
            .bearer_auth(self.api_key.as_str())
    }

    pub async fn execute_command(
        &self,
        command: impl ExecuteVultrCommand,
    ) -> Result<impl Deserialize, ServiceError> {
        command.execute(self).await
    }
}

pub fn get_vultr_client(vultr_api_key: &str) -> &'static VultrClient {
    static CLIENT: OnceLock<VultrClient> = OnceLock::new();
    CLIENT.get_or_init(|| VultrClient::new(vultr_api_key.to_string()))
}
