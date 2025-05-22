use std::str::FromStr;

use super::{
    diagrams::{
        BlockStorage, Compute, FirewallGroup, FirewallRule, ManagedDatabase, ObjectPosition,
        ObjectStorage,
    },
    enums::ResourceType,
    UserRole, VultrExecutionContext,
};
use crate::{
    adapter::{
        kv_store::{interfaces::KVStore, rocks_db::get_rocks_db},
        repositories::project::diagram::{
            delete_block_storage, delete_compute, delete_firewall_group, delete_firewall_rule,
            delete_managed_database, delete_object_storage, insert_block_storage, insert_compute,
            insert_firewall_group, insert_firewall_rule, insert_managed_database,
            insert_object_storage, update_block_storage, update_compute, update_firewall_group,
            update_managed_database, update_object_storage,
        },
        request_dispensor::vultr::{
            interfaces::{
                ExecuteVultrCreateCommand, ExecuteVultrDeleteCommand, ExecuteVultrGetCommand,
                ExecuteVultrUpdateCommand,
            },
            schemas::{
                block_storage::{
                    AttachBlockStorageToCompute, CreateBlockStorage, DeleteBlockStorage,
                    DetachBlockStorageFromCompute, GetBlockStorage, UpdateBlockStorage,
                },
                firewall::{
                    CreateFirewallGroup, CreateFirewallRule, DeleteFirewallGroup,
                    DeleteFirewallRule, GetFirewallGroup, UpdateFirewallGroup,
                },
                instance::{CreateCompute, DeleteCompute, UpdateCompute},
                managed_database::{
                    CreateManagedDatabase, DeleteManagedDatabase, UpdateManagedDatabase,
                },
                object_storage::{
                    CreateObjectStorage, DeleteObjectStorage, GetObjectStorage, UpdateObjectStorage,
                },
            },
        },
    },
    errors::ServiceError,
};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use sqlx::PgConnection;
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Debug, Clone, Deserialize, Serialize, ToSchema)]
pub(crate) struct CreateProject {
    pub(crate) name: String,
    pub(crate) description: String,
}

#[derive(Debug, Deserialize, Serialize, ToSchema)]
pub(crate) struct DeleteProject {
    pub(crate) project_id: Uuid,
}

#[derive(Debug, Deserialize, Serialize, ToSchema)]
pub(crate) struct ExpelMember {
    pub(crate) project_id: Uuid,
    pub(crate) expelled_email: String,
}

#[derive(Debug, Deserialize, ToSchema)]
pub(crate) struct AssignRole {
    pub(crate) project_id: Uuid,
    pub(crate) invitee_email: String,
    pub(crate) role: UserRole,
}

#[derive(Debug, Clone, Deserialize, Serialize, ToSchema)]
pub(crate) struct RegisterVultApiKey {
    pub(crate) project_id: Uuid,
    pub(crate) api_key: Vec<u8>,
}

#[derive(Clone, Deserialize, Serialize, ToSchema)]
pub struct CommandRequest {
    pub command_name: String,
    pub temp_id: String,
    pub position: ObjectPosition,
    pub data: Value,
}

#[derive(Debug, Clone, Deserialize, Serialize, ToSchema)]
pub struct ResourceResponse {
    pub temp_id: String,
    pub resource_type: ResourceType,
    pub position: ObjectPosition,
    pub attributes: Value,
}
impl ResourceResponse {
    pub fn into_response(
        resource_type: ResourceType,
        attributes: Value,
    ) -> Result<Self, ServiceError> {
        let y = attributes["y"]
            .as_i64()
            .ok_or_else(|| ServiceError::NotFound)?;
        let x = attributes["x"]
            .as_i64()
            .ok_or_else(|| ServiceError::NotFound)?;
        Ok(ResourceResponse {
            temp_id: "".to_string(),
            resource_type,
            position: ObjectPosition { x, y },
            attributes,
        })
    }
}

#[derive(Clone, Deserialize, Serialize, ToSchema)]
pub struct CommandList {
    pub command_list: Vec<CommandRequest>,
}

#[derive(Clone, Deserialize, Serialize, ToSchema)]
pub enum VultrCommand {
    Create {
        temp_id: String,
        position: ObjectPosition,
        data: Value,
    },
    Update {
        temp_id: String,
        position: ObjectPosition,
        data: Value,
    },
    Delete {
        temp_id: String,
        data: Value,
    },
}

impl VultrCommand {
    pub fn from_request(request: CommandRequest) -> Result<Self, ServiceError> {
        match request.command_name.as_str() {
            name if name.starts_with("Create") => Ok(VultrCommand::Create {
                temp_id: request.temp_id,
                position: request.position,
                data: request.data,
            }),
            name if name.starts_with("Update")
                || name.contains("Attach")
                || name.contains("Detach") =>
            {
                Ok(VultrCommand::Update {
                    temp_id: request.temp_id,
                    position: request.position,
                    data: request.data,
                })
            }
            name if name.starts_with("Delete") => Ok(VultrCommand::Delete {
                temp_id: request.temp_id,
                data: request.data,
            }),
            _ => Err(ServiceError::NotFound),
        }
    }

    pub fn get_command_name(&self) -> &str {
        match self {
            VultrCommand::Create { .. } => "Create",
            VultrCommand::Update { .. } => "Update",
            VultrCommand::Delete { .. } => "Delete",
        }
    }
    pub fn get_command_data(&self) -> Value {
        match self {
            VultrCommand::Create { data, .. } => data.clone(),
            VultrCommand::Update { data, .. } => data.clone(),
            VultrCommand::Delete { data, .. } => data.clone(),
        }
    }
    pub fn get_temp_id(&self) -> &String {
        match self {
            VultrCommand::Create { temp_id, .. } => temp_id,
            VultrCommand::Update { temp_id, .. } => temp_id,
            VultrCommand::Delete { temp_id, .. } => temp_id,
        }
    }
}

#[derive(Serialize, Deserialize)]
pub enum VultrCreateCommand {
    Compute(CreateCompute),
    BlockStorage(CreateBlockStorage),
    FirewallGroup(CreateFirewallGroup),
    FirewallRule(CreateFirewallRule),
    ManagedDatabase(CreateManagedDatabase),
    ObjectStorage(CreateObjectStorage),
}

impl VultrCreateCommand {
    pub async fn execute_create_command(
        self,
        context: &mut VultrExecutionContext,
        trx: &mut PgConnection,
        temp_id: &String,
    ) -> Result<(), ServiceError> {
        let result = match self {
            VultrCreateCommand::Compute(command) => {
                let res = command.execute(context.vultr_client).await?;
                let compute: Compute = serde_json::from_value(res.clone())?;
                insert_compute(&compute, trx).await?;
                res
            }
            VultrCreateCommand::BlockStorage(command) => {
                let res = command.execute(context.vultr_client).await?;
                let block_storage: BlockStorage = serde_json::from_value(res.clone())?;
                insert_block_storage(&block_storage, trx).await?;
                res
            }
            VultrCreateCommand::FirewallGroup(command) => {
                let res = command.execute(context.vultr_client).await?;
                let fire_wall_group: FirewallGroup = serde_json::from_value(res.clone())?;
                insert_firewall_group(&fire_wall_group, trx).await?;
                res
            }
            VultrCreateCommand::FirewallRule(command) => {
                let res = command.execute(context.vultr_client).await?;
                let fire_wall_rule: FirewallRule = serde_json::from_value(res.clone())?;
                insert_firewall_rule(&fire_wall_rule, trx).await?;
                res
            }
            VultrCreateCommand::ManagedDatabase(command) => {
                let res = command.execute(context.vultr_client).await?;
                let managed_database: ManagedDatabase = serde_json::from_value(res.clone())?;
                insert_managed_database(&managed_database, trx).await?;
                res
            }
            VultrCreateCommand::ObjectStorage(command) => {
                let res = command.execute(context.vultr_client).await?;
                let object_storage: ObjectStorage = serde_json::from_value(res.clone())?;
                insert_object_storage(&object_storage, trx).await?;
                res
            }
        };
        // HM에 저장
        let db = get_rocks_db().await;
        db.insert(temp_id.as_bytes(), result.to_string().as_bytes())
            .await?;

        Ok(())
    }
}

#[derive(Serialize, Deserialize)]
pub enum VultrUpdateCommand {
    UpdateCompute(UpdateCompute),
    UpdateBlockStorage(UpdateBlockStorage),
    UpdateFirewallGroup(UpdateFirewallGroup),
    UpdateManagedDatabase(UpdateManagedDatabase),
    UpdateObjectStorage(UpdateObjectStorage),
    AttachBlockStorageToCompute(AttachBlockStorageToCompute),
    DetachBlockStorageFromCompute(DetachBlockStorageFromCompute),
}

#[derive(Serialize, Deserialize, ToSchema)]
pub struct DeployProject {
    pub project_id: Uuid,
    pub command_list: Vec<CommandRequest>,
}

impl VultrUpdateCommand {
    pub async fn execute_update_command(
        mut self,
        context: &mut VultrExecutionContext,
        trx: &mut PgConnection,
        temp_id: &String,
    ) -> Result<(), ServiceError> {
        // Check if temp_id exists in HashMap
        if self.get_id().is_none() {
            let id = context.get_id_with_temp_id(temp_id)?;
            self.assign_id(id)?;
        }

        // Parse the result
        let _ = match self {
            VultrUpdateCommand::UpdateCompute(command) => {
                let res = command.execute(context.vultr_client).await?.unwrap(); // return Some
                let compute: Compute = serde_json::from_value(res.clone())?;
                update_compute(&compute, trx).await?;
                res
            }
            VultrUpdateCommand::UpdateBlockStorage(command) => {
                let block_id = command.id.ok_or_else(|| ServiceError::NotFound)?;
                let _ = command.execute(context.vultr_client).await?;
                let block_storage = GetBlockStorage::new(block_id)
                    .execute(context.vultr_client)
                    .await?;
                update_block_storage(&block_storage, trx).await?;
                json!(block_storage)
            }
            VultrUpdateCommand::AttachBlockStorageToCompute(command) => {
                let id = command.id.ok_or_else(|| ServiceError::NotFound)?;
                let _ = command.execute(context.vultr_client).await?;
                let block_storage = GetBlockStorage::new(id)
                    .execute(context.vultr_client)
                    .await?;
                update_block_storage(&block_storage, trx).await?;
                json!(block_storage)
            }
            VultrUpdateCommand::DetachBlockStorageFromCompute(command) => {
                let id = command.id.ok_or_else(|| ServiceError::NotFound)?;
                let _ = command.execute(context.vultr_client).await?;
                let block_storage = GetBlockStorage::new(id)
                    .execute(context.vultr_client)
                    .await?;
                update_block_storage(&block_storage, trx).await?;
                json!(block_storage)
            }
            VultrUpdateCommand::UpdateFirewallGroup(command) => {
                let firewall_group_id = command.id.ok_or_else(|| ServiceError::NotFound)?;
                let _ = command.execute(context.vultr_client).await?;
                let firewall_group = GetFirewallGroup::new(firewall_group_id)
                    .execute(context.vultr_client)
                    .await?;
                update_firewall_group(&firewall_group, trx).await?;
                json!(firewall_group)
            }
            VultrUpdateCommand::UpdateManagedDatabase(command) => {
                let res = command.execute(context.vultr_client).await?.unwrap(); // return Some
                let managed_database: ManagedDatabase = serde_json::from_value(res.clone())?;
                update_managed_database(&managed_database, trx).await?;
                json!(managed_database)
            }
            VultrUpdateCommand::UpdateObjectStorage(command) => {
                let object_storage_id = command.id.ok_or_else(|| ServiceError::NotFound)?;
                let _ = command.execute(context.vultr_client).await?;
                let object_storage = GetObjectStorage::new(object_storage_id)
                    .execute(context.vultr_client)
                    .await?;
                update_object_storage(&object_storage, trx).await?;
                json!(object_storage)
            }
        };

        // Update the data in Vultr
        Ok(())
    }

    fn get_id(&self) -> Option<Uuid> {
        match self {
            VultrUpdateCommand::UpdateCompute(command) => command.id,
            VultrUpdateCommand::UpdateBlockStorage(command) => command.id,
            VultrUpdateCommand::AttachBlockStorageToCompute(command) => command.id,
            VultrUpdateCommand::DetachBlockStorageFromCompute(command) => command.id,
            VultrUpdateCommand::UpdateFirewallGroup(command) => command.id,
            VultrUpdateCommand::UpdateManagedDatabase(command) => command.id,
            VultrUpdateCommand::UpdateObjectStorage(command) => command.id,
        }
    }
    fn assign_id(&mut self, id: String) -> Result<(), ServiceError> {
        let id = Uuid::from_str(&id).map_err(|_| {
            ServiceError::ParsingError(Box::new(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                "Invalid UUID",
            )))
        })?;
        match self {
            VultrUpdateCommand::UpdateCompute(command) => {
                command.id = Some(id);
            }
            VultrUpdateCommand::UpdateBlockStorage(command) => {
                command.id = Some(id);
            }
            VultrUpdateCommand::AttachBlockStorageToCompute(command) => {
                command.id = Some(id);
            }
            VultrUpdateCommand::DetachBlockStorageFromCompute(command) => {
                command.id = Some(id);
            }
            VultrUpdateCommand::UpdateFirewallGroup(command) => {
                command.id = Some(id);
            }
            VultrUpdateCommand::UpdateManagedDatabase(command) => {
                command.id = Some(id);
            }
            VultrUpdateCommand::UpdateObjectStorage(command) => {
                command.id = Some(id);
            }
        }
        Ok(())
    }
}

#[derive(Serialize, Deserialize)]
pub enum VultrDeleteCommand {
    Compute(DeleteCompute),
    BlockStorage(DeleteBlockStorage),
    FirewallGroup(DeleteFirewallGroup),
    FirewallRule(DeleteFirewallRule),
    ManagedDatabase(DeleteManagedDatabase),
    ObjectStorage(DeleteObjectStorage),
}

impl VultrDeleteCommand {
    pub async fn execute_delete_command(
        mut self,
        context: &mut VultrExecutionContext,
        trx: &mut PgConnection,
        temp_id: &String,
    ) -> Result<(), ServiceError> {
        if self.id_is_none() {
            let id = context.get_id_with_temp_id(temp_id)?;
            self.assign_id(id)?;
        }
        match self {
            VultrDeleteCommand::Compute(command) => {
                let command_id = command.id.unwrap();
                command.execute(context.vultr_client).await?;
                delete_compute(&context.project_id, &command_id, trx).await?;
            }
            VultrDeleteCommand::BlockStorage(command) => {
                let block_id = command.id.unwrap();
                command.execute(context.vultr_client).await?;
                delete_block_storage(&context.project_id, &block_id, trx).await?;
            }
            VultrDeleteCommand::FirewallGroup(command) => {
                let firewall_group_id = command.id.unwrap();
                command.execute(context.vultr_client).await?;
                delete_firewall_group(&context.project_id, &firewall_group_id, trx).await?;
            }
            VultrDeleteCommand::FirewallRule(command) => {
                let fire_wall_rule_id = command.fire_wall_rule_id.unwrap();
                command.execute(context.vultr_client).await?;
                delete_firewall_rule(&context.project_id, &fire_wall_rule_id, trx).await?;
            }
            VultrDeleteCommand::ManagedDatabase(command) => {
                let managed_database_id = command.id.unwrap();
                command.execute(context.vultr_client).await?;
                delete_managed_database(&context.project_id, &managed_database_id, trx).await?;
            }
            VultrDeleteCommand::ObjectStorage(command) => {
                let object_storage_id = command.id.unwrap();
                command.execute(context.vultr_client).await?;
                delete_object_storage(&context.project_id, &object_storage_id, trx).await?;
            }
        }
        Ok(())
    }
    fn id_is_none(&self) -> bool {
        match self {
            VultrDeleteCommand::Compute(command) => command.id.is_none(),
            VultrDeleteCommand::BlockStorage(command) => command.id.is_none(),
            VultrDeleteCommand::FirewallGroup(command) => command.id.is_none(),
            VultrDeleteCommand::FirewallRule(command) => command.fire_wall_rule_id.is_none(),
            VultrDeleteCommand::ManagedDatabase(command) => command.id.is_none(),
            VultrDeleteCommand::ObjectStorage(command) => command.id.is_none(),
        }
    }
    fn assign_id(&mut self, id: String) -> Result<(), ServiceError> {
        match self {
            VultrDeleteCommand::Compute(command) => {
                let id = Uuid::from_str(&id).map_err(|_| {
                    ServiceError::ParsingError(Box::new(std::io::Error::new(
                        std::io::ErrorKind::InvalidData,
                        "Invalid UUID",
                    )))
                })?;
                command.id = Some(id);
            }
            VultrDeleteCommand::BlockStorage(command) => {
                let id = Uuid::from_str(&id).map_err(|_| {
                    ServiceError::ParsingError(Box::new(std::io::Error::new(
                        std::io::ErrorKind::InvalidData,
                        "Invalid UUID",
                    )))
                })?;
                command.id = Some(id);
            }
            VultrDeleteCommand::FirewallGroup(command) => {
                let id = Uuid::from_str(&id).map_err(|_| {
                    ServiceError::ParsingError(Box::new(std::io::Error::new(
                        std::io::ErrorKind::InvalidData,
                        "Invalid UUID",
                    )))
                })?;
                command.id = Some(id);
            }
            VultrDeleteCommand::FirewallRule(command) => {
                let id = i64::from_str(&id).map_err(|_| {
                    ServiceError::ParsingError(Box::new(std::io::Error::new(
                        std::io::ErrorKind::InvalidData,
                        "Invalid i64",
                    )))
                })?;
                command.fire_wall_rule_id = Some(id);
            }
            VultrDeleteCommand::ManagedDatabase(command) => {
                let id = Uuid::from_str(&id).map_err(|_| {
                    ServiceError::ParsingError(Box::new(std::io::Error::new(
                        std::io::ErrorKind::InvalidData,
                        "Invalid UUID",
                    )))
                })?;
                command.id = Some(id);
            }
            VultrDeleteCommand::ObjectStorage(command) => {
                let id = Uuid::from_str(&id).map_err(|_| {
                    ServiceError::ParsingError(Box::new(std::io::Error::new(
                        std::io::ErrorKind::InvalidData,
                        "Invalid UUID",
                    )))
                })?;
                command.id = Some(id);
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    use uuid::Uuid;

    #[test]
    fn test_command_request_deserialize() {
        let json = json!({
            "command_name": "CreateCompute",
            "temp_id": "temp1",
            "position": {
                "x": 20,
                "y": 20
            },
            "data": {
                "id": "temp1",
                "project_id": Uuid::new_v4().to_string(),
                "name": "test-compute"
            }
        });

        let command: CommandRequest = serde_json::from_value(json).unwrap();
        assert_eq!(command.command_name, "CreateCompute");
        assert_eq!(command.position.x, 20);
        assert_eq!(command.position.y, 20);
        assert_eq!(command.data["id"], "temp1");
    }

    #[test]
    fn test_command_list_deserialize() {
        let project_id = Uuid::new_v4();
        let json = json!({
            "command_list": [
                {
                    "command_name": "CreateCompute",
                    "temp_id": "temp1",
                    "position": {
                        "x": 20,
                        "y": 20
                    },
                    "data": {
                        "id": "temp1",
                        "project_id": project_id.to_string(),
                        "name": "test-compute-1"
                    }
                },
                {
                    "command_name": "UpdateCompute",
                    "temp_id": "temp2",
                    "position": {
                        "x": 30,
                        "y": 30
                    },
                    "data": {
                        "id": "temp2",
                        "project_id": project_id.to_string(),
                        "name": "test-compute-2"
                    }
                }
            ]
        });

        let command_list: CommandList = serde_json::from_value(json).unwrap();
        assert_eq!(command_list.command_list.len(), 2);

        let first_command = &command_list.command_list[0];
        assert_eq!(first_command.command_name, "CreateCompute");
        assert_eq!(first_command.position.x, 20);
        assert_eq!(first_command.position.y, 20);
        assert_eq!(first_command.data["id"], "temp1");
        assert_eq!(first_command.data["project_id"], project_id.to_string());

        let second_command = &command_list.command_list[1];
        assert_eq!(second_command.command_name, "UpdateCompute");
        assert_eq!(second_command.position.x, 30);
        assert_eq!(second_command.position.y, 30);
        assert_eq!(second_command.data["id"], "temp2");
        assert_eq!(second_command.data["project_id"], project_id.to_string());
    }

    #[test]
    fn test_vultr_command_from_request() {
        let project_id = Uuid::new_v4();
        let id = Uuid::new_v4();
        let request = CommandRequest {
            command_name: "CreateCompute".to_string(),
            temp_id: "temp1".to_string(),
            position: ObjectPosition { x: 20, y: 20 },
            data: json!({
                "id": id.to_string(),
                "project_id": project_id.to_string(),
                "name": "test-compute"
            }),
        };

        let command = VultrCommand::from_request(request);
        match command {
            Ok(VultrCommand::Create {
                temp_id,
                position,
                data,
            }) => {
                assert_eq!(temp_id, "temp1");
                assert_eq!(position.x, 20);
                assert_eq!(position.y, 20);
                assert_eq!(data["id"], id.to_string());
                assert_eq!(data["project_id"], project_id.to_string());
            }
            _ => panic!("Expected Create variant"),
        }
    }

    #[test]
    fn test_vultr_command_from_request_invalid() {
        let project_id = Uuid::new_v4();
        let id = Uuid::new_v4();
        let request = CommandRequest {
            command_name: "InvalidCommand".to_string(),
            temp_id: "temp1".to_string(),
            position: ObjectPosition { x: 20, y: 20 },
            data: json!({
                "id": id.to_string(),
                "project_id": project_id.to_string(),
                "name": "test-compute"
            }),
        };

        assert!(matches!(
            VultrCommand::from_request(request),
            Err(ServiceError::NotFound)
        ));
    }
}
