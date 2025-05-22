use std::string::FromUtf8Error;

use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::{
    adapter::request_dispensor::vultr::VultrClient,
    domain::project::commands::{
        VultrCommand, VultrCreateCommand, VultrDeleteCommand, VultrUpdateCommand,
    },
    errors::ServiceError,
};

use super::{
    block_storage::{CreateBlockStorage, DeleteBlockStorage, UpdateBlockStorage},
    firewall::{
        self, CreateFirewallGroup, CreateFirewallRule, DeleteFirewallGroup, DeleteFirewallRule,
        UpdateFirewallGroup,
    },
    instance::{CreateCompute, DeleteCompute, UpdateCompute},
    managed_database::{CreateManagedDatabase, DeleteManagedDatabase, UpdateManagedDatabase},
    object_storage::{CreateObjectStorage, DeleteObjectStorage, UpdateObjectStorage},
};

impl From<reqwest::Error> for ServiceError {
    fn from(error: reqwest::Error) -> Self {
        tracing::error!("Request error while sending request to Vultr: {:?}", error);
        ServiceError::RequestError(Box::new(error))
    }
}

impl From<serde_json::Error> for ServiceError {
    fn from(_: serde_json::Error) -> Self {
        ServiceError::ParseError
    }
}

impl From<FromUtf8Error> for ServiceError {
    fn from(error: FromUtf8Error) -> Self {
        tracing::error!("Failed to convert from UTF-8: {:?}", error);
        ServiceError::ParseError
    }
}
