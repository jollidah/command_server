use reqwest::Method;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use uuid::Uuid;

use super::extract_schema_from_response;
use crate::{
    adapter::request_dispensor::vultr::{
        interfaces::{
            ExecuteVultrCreateCommand, ExecuteVultrDeleteCommand, ExecuteVultrGetCommand,
            ExecuteVultrUpdateCommand,
        },
        VultrClient,
    },
    domain::project::{
        diagrams::FirewallGroup,
        enums::{IpType, Protocol},
    },
    errors::ServiceError,
};

#[derive(Serialize, Deserialize)]
pub struct CreateFirewallGroup {
    description: String,
}
#[derive(Serialize)]
pub struct ListFirewallGroup;

#[derive(Serialize)]
pub struct GetFirewallGroup {
    id: Uuid,
}
impl GetFirewallGroup {
    pub fn new(id: Uuid) -> Self {
        Self { id }
    }
}
#[derive(Serialize, Deserialize)]
pub struct UpdateFirewallGroup {
    pub id: Uuid,
    description: String,
}
#[derive(Serialize, Deserialize)]
pub struct DeleteFirewallGroup {
    // This id can be None if the id is not assigned yet
    pub id: Uuid,
}
#[derive(Serialize, Deserialize)]
pub struct CreateFirewallRule {
    #[serde(skip_serializing)]
    pub firewall_group_id: Uuid, // Use id as path parameter
    ip_type: IpType,
    protocol: Protocol,
    port: String,
    subnet: String,
    subnet_size: i64,
    notes: String,
}

#[derive(Serialize)]
pub struct ListFirewallRule {
    firewall_group_id: Uuid,
}

#[derive(Serialize, Deserialize)]
pub struct DeleteFirewallRule {
    pub firewall_group_id: Uuid,
    // This id can be None if the id is not assigned yet
    pub firewall_rule_id: i64,
}

#[derive(Serialize)]
pub struct GetFirewallRule {
    firewall_group_id: Uuid,
    firewall_rule_id: i64,
}

#[allow(refining_impl_trait)]
impl ExecuteVultrCreateCommand for CreateFirewallGroup {
    async fn execute(self, vultr_client: &VultrClient) -> Result<Value, ServiceError> {
        let response = vultr_client
            .build_request(Method::POST, "firewalls".to_string())
            .json(&serde_json::json!(self))
            .send()
            .await?;
        extract_schema_from_response::<Value>(response, "firewall_group").await
    }
}

#[allow(refining_impl_trait)]
impl ExecuteVultrGetCommand for GetFirewallGroup {
    async fn execute(self, vultr_client: &VultrClient) -> Result<FirewallGroup, ServiceError> {
        let response = vultr_client
            .build_request(Method::GET, format!("firewalls/{}", self.id))
            .send()
            .await?;
        extract_schema_from_response::<FirewallGroup>(response, "firewall_group").await
    }
}

#[allow(refining_impl_trait)]
impl ExecuteVultrUpdateCommand for UpdateFirewallGroup {
    async fn execute(self, vultr_client: &VultrClient) -> Result<Option<Value>, ServiceError> {
        vultr_client
            .build_request(Method::PUT, format!("firewalls/{}", self.id))
            .json(&serde_json::json!(self))
            .send()
            .await?;

        Ok(None)
    }
}

#[allow(refining_impl_trait)]
impl ExecuteVultrDeleteCommand for DeleteFirewallGroup {
    async fn execute(self, vultr_client: &VultrClient) -> Result<(), ServiceError> {
        vultr_client
            .build_request(Method::DELETE, format!("firewalls/{}", self.id))
            .send()
            .await?;

        Ok(())
    }
}

#[allow(refining_impl_trait)]
impl ExecuteVultrCreateCommand for CreateFirewallRule {
    async fn execute(self, vultr_client: &VultrClient) -> Result<Value, ServiceError> {
        let response = vultr_client
            .build_request(
                Method::POST,
                format!("firewalls/{}/rules", self.firewall_group_id),
            )
            .json(&serde_json::json!(self))
            .send()
            .await?;

        extract_schema_from_response::<Value>(response, "firewall_rule").await
    }
}

#[allow(refining_impl_trait)]
impl ExecuteVultrDeleteCommand for DeleteFirewallRule {
    async fn execute(self, vultr_client: &VultrClient) -> Result<(), ServiceError> {
        vultr_client
            .build_request(
                Method::DELETE,
                format!(
                    "firewalls/{}/rules/{}",
                    self.firewall_group_id, self.firewall_rule_id
                ),
            )
            .send()
            .await?;

        Ok(())
    }
}
