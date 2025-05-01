use chrono::{DateTime, Utc};
use reqwest::Method;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::extract_schema_from_response;
use crate::{
    adapter::request_dispensor::vultr::{interfaces::ExecuteVultrCommand, VultrClient},
    errors::ServiceError,
};

#[derive(Debug, Serialize, Deserialize)]
pub struct FirewallGroup {
    id: Uuid,
    description: String,
    date_created: DateTime<Utc>,
    date_modified: DateTime<Utc>,
    instance_count: i64,
    rule_count: i64,
    max_rule_count: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FirewallRule {
    id: i64,
    #[serde(rename = "type")]
    rule_type: String, // "type"은 Rust 키워드이므로 이름 변경
    ip_type: String,
    action: String,
    protocol: String,
    port: String,
    subnet: String,
    subnet_size: i64,
    source: String,
    notes: String,
}

pub struct FireWallCommandFactory;
impl FireWallCommandFactory {
    pub fn create_firewall_group(description: String) -> CreateFirewallGroup {
        CreateFirewallGroup { description }
    }
    pub fn list_firewall_group() -> ListFirewallGroup {
        ListFirewallGroup {}
    }
    pub fn get_firewall_group(id: Uuid) -> GetFirewallGroup {
        GetFirewallGroup { id }
    }
    pub fn update_firewall_group(id: Uuid, description: String) -> UpdateFirewallGroup {
        UpdateFirewallGroup { id, description }
    }
    pub fn delete_firewall_group(id: Uuid) -> DeleteFirewallGroup {
        DeleteFirewallGroup { id }
    }
    pub fn create_firewall_rule(
        fire_wall_group_id: Uuid,
        ip_type: IpType,
        protocol: Protocol,
        port: String,
        subnet: String,
        subnet_size: i64,
        source: String,
        notes: String,
    ) -> CreateFirewallRule {
        CreateFirewallRule {
            ip_type,
            protocol,
            port,
            subnet,
            subnet_size,
            source,
            notes,
            fire_wall_group_id,
        }
    }
    pub fn list_firewall_rule(fire_wall_group_id: Uuid) -> ListFirewallRule {
        ListFirewallRule { fire_wall_group_id }
    }
    pub fn delete_firewall_rule(
        fire_wall_group_id: Uuid,
        fire_wall_rule_id: i64,
    ) -> DeleteFirewallRule {
        DeleteFirewallRule {
            fire_wall_group_id,
            fire_wall_rule_id,
        }
    }
    pub fn get_firewall_rule(fire_wall_group_id: Uuid, fire_wall_rule_id: i64) -> GetFirewallRule {
        GetFirewallRule {
            fire_wall_group_id,
            fire_wall_rule_id,
        }
    }
}

#[derive(Serialize)]
pub struct CreateFirewallGroup {
    description: String,
}
#[derive(Serialize)]
pub struct ListFirewallGroup;

#[derive(Serialize)]
pub struct GetFirewallGroup {
    id: Uuid,
}
#[derive(Serialize)]
pub struct UpdateFirewallGroup {
    id: Uuid,
    description: String,
}
#[derive(Serialize)]
pub struct DeleteFirewallGroup {
    id: Uuid,
}
#[derive(Serialize)]
#[serde(rename_all = "lowercase")]
pub enum IpType {
    V4,
    V6,
}
#[derive(Serialize)]
#[serde(rename_all = "lowercase")]
pub enum Protocol {
    Icmp,
    Tcp,
    Udp,
    Gre,
    Esp,
    Ah,
}
#[derive(Serialize)]
pub struct CreateFirewallRule {
    #[serde(skip_serializing)]
    fire_wall_group_id: Uuid,

    ip_type: IpType,
    protocol: Protocol,
    port: String,
    subnet: String,
    subnet_size: i64,
    source: String,
    notes: String,
}

#[derive(Serialize)]
pub struct ListFirewallRule {
    fire_wall_group_id: Uuid,
}

#[derive(Serialize)]
pub struct DeleteFirewallRule {
    fire_wall_group_id: Uuid,
    fire_wall_rule_id: i64,
}

#[derive(Serialize)]
pub struct GetFirewallRule {
    fire_wall_group_id: Uuid,
    fire_wall_rule_id: i64,
}

#[allow(refining_impl_trait)]
impl ExecuteVultrCommand for CreateFirewallGroup {
    async fn execute(self, vultr_client: &VultrClient) -> Result<FirewallGroup, ServiceError> {
        let response = vultr_client
            .build_request(Method::POST, "firewalls".to_string())
            .json(&serde_json::json!(self))
            .send()
            .await?;
        extract_schema_from_response::<FirewallGroup>(response, "firewall_group").await
    }
}

#[allow(refining_impl_trait)]
impl ExecuteVultrCommand for ListFirewallGroup {
    async fn execute(self, vultr_client: &VultrClient) -> Result<Vec<FirewallGroup>, ServiceError> {
        let response = vultr_client
            .build_request(Method::GET, "firewalls".to_string())
            .send()
            .await?;
        extract_schema_from_response::<Vec<FirewallGroup>>(response, "firewall_groups").await
    }
}

#[allow(refining_impl_trait)]
impl ExecuteVultrCommand for GetFirewallGroup {
    async fn execute(self, vultr_client: &VultrClient) -> Result<FirewallGroup, ServiceError> {
        let response = vultr_client
            .build_request(Method::GET, format!("firewalls/{}", self.id))
            .send()
            .await?;
        println!("{:?}", response);
        extract_schema_from_response::<FirewallGroup>(response, "firewall_group").await
    }
}

#[allow(refining_impl_trait)]
impl ExecuteVultrCommand for UpdateFirewallGroup {
    async fn execute(self, vultr_client: &VultrClient) -> Result<(), ServiceError> {
        vultr_client
            .build_request(Method::PUT, format!("firewalls/{}", self.id))
            .json(&serde_json::json!(self))
            .send()
            .await?;

        Ok(())
    }
}

#[allow(refining_impl_trait)]
impl ExecuteVultrCommand for DeleteFirewallGroup {
    async fn execute(self, vultr_client: &VultrClient) -> Result<(), ServiceError> {
        vultr_client
            .build_request(Method::DELETE, format!("firewalls/{}", self.id))
            .send()
            .await?;

        Ok(())
    }
}

#[allow(refining_impl_trait)]
impl ExecuteVultrCommand for CreateFirewallRule {
    async fn execute(self, vultr_client: &VultrClient) -> Result<FirewallRule, ServiceError> {
        let response = vultr_client
            .build_request(
                Method::POST,
                format!("firewalls/{}/rules", self.fire_wall_group_id),
            )
            .json(&serde_json::json!(self))
            .send()
            .await?;

        extract_schema_from_response::<FirewallRule>(response, "firewall_rule").await
    }
}

#[allow(refining_impl_trait)]
impl ExecuteVultrCommand for ListFirewallRule {
    async fn execute(self, vultr_client: &VultrClient) -> Result<Vec<FirewallRule>, ServiceError> {
        let response = vultr_client
            .build_request(
                Method::GET,
                format!("firewalls/{}/rules", self.fire_wall_group_id),
            )
            .send()
            .await?;

        extract_schema_from_response::<Vec<FirewallRule>>(response, "firewall_rules").await
    }
}

#[allow(refining_impl_trait)]
impl ExecuteVultrCommand for DeleteFirewallRule {
    async fn execute(self, vultr_client: &VultrClient) -> Result<(), ServiceError> {
        vultr_client
            .build_request(
                Method::DELETE,
                format!(
                    "firewalls/{}/rules/{}",
                    self.fire_wall_group_id, self.fire_wall_rule_id
                ),
            )
            .send()
            .await?;

        Ok(())
    }
}

#[allow(refining_impl_trait)]
impl ExecuteVultrCommand for GetFirewallRule {
    async fn execute(self, vultr_client: &VultrClient) -> Result<FirewallRule, ServiceError> {
        let response = vultr_client
            .build_request(
                Method::GET,
                format!(
                    "firewalls/{}/rules/{}",
                    self.fire_wall_group_id, self.fire_wall_rule_id
                ),
            )
            .send()
            .await?;

        extract_schema_from_response::<FirewallRule>(response, "firewall_rule").await
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use super::*;
    use crate::adapter::request_dispensor::vultr::get_vultr_client;
    use crate::config::get_config;

    #[tokio::test]
    async fn test_create_firewall_group() {
        let vultr_client = get_vultr_client(&get_config().vultr_api_key);
        let firewall_group = FireWallCommandFactory::create_firewall_group("test".to_string())
            .execute(&vultr_client)
            .await
            .unwrap();
        println!("{:?}", firewall_group);
    }

    #[tokio::test]
    async fn test_list_firewall_group() {
        let vultr_client = get_vultr_client(&get_config().vultr_api_key);
        let firewall_groups = FireWallCommandFactory::list_firewall_group()
            .execute(&vultr_client)
            .await
            .unwrap();
        println!("{:?}", firewall_groups);
    }

    #[tokio::test]
    async fn test_get_firewall_group() {
        let vultr_client = get_vultr_client(&get_config().vultr_api_key);
        let firewall_group = FireWallCommandFactory::get_firewall_group(
            Uuid::from_str("21264f8b-c28d-4e82-83b7-3ad0b4b953ec").unwrap(),
        )
        .execute(&vultr_client)
        .await
        .unwrap();
        println!("{:?}", firewall_group);
    }

    #[tokio::test]
    async fn test_update_firewall_group() {
        let vultr_client = get_vultr_client(&get_config().vultr_api_key);
        let firewall_group =
            FireWallCommandFactory::update_firewall_group(Uuid::new_v4(), "test".to_string())
                .execute(&vultr_client)
                .await
                .unwrap();
        println!("{:?}", firewall_group);
    }

    #[tokio::test]
    async fn test_delete_firewall_group() {
        let vultr_client = get_vultr_client(&get_config().vultr_api_key);
        let firewall_group = FireWallCommandFactory::delete_firewall_group(Uuid::new_v4())
            .execute(&vultr_client)
            .await
            .unwrap();
        println!("{:?}", firewall_group);
    }

    #[tokio::test]
    async fn test_create_firewall_rule() {
        let vultr_client = get_vultr_client(&get_config().vultr_api_key);
        let firewall_rule = FireWallCommandFactory::create_firewall_rule(
            Uuid::new_v4(),
            IpType::V4,
            Protocol::Tcp,
            "80".to_string(),
            "0.0.0.0/0".to_string(),
            32,
            "192.168.1.1".to_string(),
            "test".to_string(),
        )
        .execute(&vultr_client)
        .await
        .unwrap();
        println!("{:?}", firewall_rule);
    }

    #[tokio::test]
    async fn test_list_firewall_rule() {
        let vultr_client = get_vultr_client(&get_config().vultr_api_key);
        let firewall_rules = FireWallCommandFactory::list_firewall_rule(Uuid::new_v4())
            .execute(&vultr_client)
            .await
            .unwrap();
        println!("{:?}", firewall_rules);
    }

    #[tokio::test]
    async fn test_delete_firewall_rule() {
        let vultr_client = get_vultr_client(&get_config().vultr_api_key);
        let firewall_rule = FireWallCommandFactory::delete_firewall_rule(Uuid::new_v4(), 1)
            .execute(&vultr_client)
            .await
            .unwrap();
        println!("{:?}", firewall_rule);
    }

    #[tokio::test]
    async fn test_get_firewall_rule() {
        let vultr_client = get_vultr_client(&get_config().vultr_api_key);
        let firewall_rule = FireWallCommandFactory::get_firewall_rule(Uuid::new_v4(), 1)
            .execute(&vultr_client)
            .await
            .unwrap();
        println!("{:?}", firewall_rule);
    }
}
