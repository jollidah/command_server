use chrono::{DateTime, Utc};
use reqwest::Method;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{
    adapter::request_dispensor::vultr::{interfaces::ExecuteVultrCommand, VultrClient},
    errors::ServiceError,
};

use super::extract_schema_from_response;

#[derive(Serialize, Deserialize)]
pub struct Instance {
    id: Uuid,
    os: String,
    ram: i64,
    disk: i64,
    main_ip: String,
    vcpu_count: i64,
    region: String,
    plan: String,
    date_created: DateTime<Utc>,
    status: String,
    allowed_bandwidth: i64,
    netmask_v4: String,
    gateway_v4: String,
    power_status: String,
    server_status: String,
    v6_network: String,
    v6_main_ip: String,
    v6_network_size: i64,
    label: String,
    internal_ip: String,
    kvm: String,
    hostname: String,
    os_id: i64,
    app_id: i64,
    image_id: String,
    firewall_group_id: String,
    features: Vec<String>,
    tags: Vec<String>,
    user_scheme: String,
    pending_charges: f64,
}

pub struct InstanceCommandFactory;
impl InstanceCommandFactory {
    pub fn list_instance() -> ListInstance {
        ListInstance {}
    }
    pub fn create_instance(
        region: String,
        plan: String,
        label: String,
        os_id: i64,
        backups: BackupStatus,
        hostname: String,
    ) -> CreateInstance {
        CreateInstance {
            region,
            plan,
            label,
            os_id,
            backups,
            hostname,
        }
    }
    pub fn get_instance(id: Uuid) -> GetInstance {
        GetInstance { id }
    }
    #[allow(clippy::too_many_arguments)]
    pub fn update_instance(
        id: Uuid,
        backups: BackupStatus,
        firewall_group_id: String,
        os_id: i64,
        plan: String,
        ddos_protection: bool,
        label: String,
        tags: Vec<String>,
    ) -> UpdateInstance {
        UpdateInstance {
            id,
            backups,
            firewall_group_id,
            os_id,
            plan,
            ddos_protection,
            label,
            tags,
        }
    }
    pub fn delete_instance(id: Uuid) -> DeleteInstance {
        DeleteInstance { id }
    }
}
#[derive(Serialize)]
pub enum BackupStatus {
    Enabled,
    Disabled,
}
#[derive(Serialize)]
pub struct ListInstance;
#[derive(Serialize)]
pub struct CreateInstance {
    region: String,
    plan: String,
    label: String,
    os_id: i64,
    backups: BackupStatus,
    hostname: String,
}
#[derive(Serialize)]
pub struct GetInstance {
    id: Uuid,
}
#[derive(Serialize)]
pub struct UpdateInstance {
    #[serde(skip_serializing)]
    id: Uuid,
    backups: BackupStatus,
    firewall_group_id: String,
    os_id: i64,
    plan: String,
    ddos_protection: bool,
    label: String,
    tags: Vec<String>,
}
#[derive(Serialize)]
pub struct DeleteInstance {
    id: Uuid,
}
#[allow(refining_impl_trait)]
impl ExecuteVultrCommand for ListInstance {
    async fn execute(self, vultr_client: &VultrClient) -> Result<Vec<Instance>, ServiceError> {
        let response = vultr_client
            .build_request(Method::GET, "instances".to_string())
            .send()
            .await?;
        extract_schema_from_response::<Vec<Instance>>(response, "instances").await
    }
}

#[allow(refining_impl_trait)]
impl ExecuteVultrCommand for CreateInstance {
    async fn execute(self, vultr_client: &VultrClient) -> Result<Instance, ServiceError> {
        let response = vultr_client
            .build_request(Method::POST, "instances".to_string())
            .send()
            .await?;
        extract_schema_from_response::<Instance>(response, "instance").await
    }
}

#[allow(refining_impl_trait)]
impl ExecuteVultrCommand for GetInstance {
    async fn execute(self, vultr_client: &VultrClient) -> Result<Instance, ServiceError> {
        let response = vultr_client
            .build_request(Method::GET, format!("instances/{}", self.id))
            .send()
            .await?;
        extract_schema_from_response::<Instance>(response, "instance").await
    }
}

#[allow(refining_impl_trait)]
impl ExecuteVultrCommand for UpdateInstance {
    async fn execute(self, vultr_client: &VultrClient) -> Result<Instance, ServiceError> {
        let response = vultr_client
            .build_request(Method::PUT, format!("instances/{}", self.id))
            .send()
            .await?;
        extract_schema_from_response::<Instance>(response, "instance").await
    }
}

#[allow(refining_impl_trait)]
impl ExecuteVultrCommand for DeleteInstance {
    async fn execute(self, vultr_client: &VultrClient) -> Result<(), ServiceError> {
        vultr_client
            .build_request(Method::DELETE, format!("instances/{}", self.id))
            .send()
            .await?;
        Ok(())
    }
}
