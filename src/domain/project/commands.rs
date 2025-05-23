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
                    CreateManagedDatabase, DeleteManagedDatabase, UpdateManagedDatabase
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
use serde_json::Value;
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
    pub(crate) api_key: String,
    // pub(crate) api_key: Vec<u8>,
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

impl CommandList {
    pub async fn execute(self, context: &mut VultrExecutionContext, trx: &mut PgConnection) -> Result<(), ServiceError> {
        for request in self.command_list {
            match request.command_name.as_str() {
                name if name.contains("Create") => {
                    let id = match request.command_name.as_str() {
                        name if name.contains("CreateCompute") => {
                            let command: CreateCompute = serde_json::from_value(request.data)?;
                            let res = command.execute(context.vultr_client).await?;
                            let compute: Compute = serde_json::from_value(res.clone())?;
                            insert_compute(&compute, trx).await?;
                            res.get("id").ok_or_else(|| ServiceError::NotFound)?.to_string()
                        }
                        name if name.contains("CreateBlockStorage") => {
                            let command: CreateBlockStorage = serde_json::from_value(request.data)?;
                            let res = command.execute(context.vultr_client).await?;
                            let block_storage: BlockStorage = serde_json::from_value(res.clone())?;
                            insert_block_storage(&block_storage, trx).await?;
                            res.get("id").ok_or_else(|| ServiceError::NotFound)?.to_string()
                        }
                        name if name.contains("CreateFirewallGroup") => {
                            let command: CreateFirewallGroup = serde_json::from_value(request.data)?;
                            let res = command.execute(context.vultr_client).await?;
                            let firewall_group: FirewallGroup = serde_json::from_value(res.clone())?;
                            insert_firewall_group(&firewall_group, trx).await?;
                            res.get("id").ok_or_else(|| ServiceError::NotFound)?.to_string()
                        }
                        name if name.contains("CreateFirewallRule") => {
                            let command: CreateFirewallRule = serde_json::from_value(request.data)?;
                            let res = command.execute(context.vultr_client).await?;
                            let firewall_rule: FirewallRule = serde_json::from_value(res.clone())?;
                            insert_firewall_rule(&firewall_rule, trx).await?;
                            res.get("id").ok_or_else(|| ServiceError::NotFound)?.to_string()
                        }
                        name if name.contains("CreateManagedDatabase") => {
                            let command: CreateManagedDatabase = serde_json::from_value(request.data)?;
                            let res = command.execute(context.vultr_client).await?;
                            let managed_database: ManagedDatabase = serde_json::from_value(res.clone())?;
                            insert_managed_database(&managed_database, trx).await?;
                            res.get("id").ok_or_else(|| ServiceError::NotFound)?.to_string()
                        }
                        name if name.contains("CreateObjectStorage") => {
                            let command: CreateObjectStorage = serde_json::from_value(request.data)?;
                            let res = command.execute(context.vultr_client).await?;
                            let object_storage: ObjectStorage = serde_json::from_value(res.clone())?;
                            insert_object_storage(&object_storage, trx).await?;
                            res.get("id").ok_or_else(|| ServiceError::NotFound)?.to_string()
                        }
                        _ => return Err(ServiceError::NotFound),
                    };
                    context.resource_map.insert(request.temp_id, id);
                }
                name if name.contains("Update") => {
                    let id = context.get_id_with_temp_id(&request.temp_id)?;
                    match request.command_name.as_str() {
                        name if name.contains("UpdateCompute") => {
                            let command: UpdateCompute = serde_json::from_value(request.data)?;
                            let res = command.execute(context.vultr_client).await?.unwrap();
                            let compute: Compute = serde_json::from_value(res.clone())?;
                            update_compute(&compute, trx).await?;
                        }
                        name if name.contains("UpdateBlockStorage") => {
                            let command: UpdateBlockStorage = serde_json::from_value(request.data)?;
                            command.execute(context.vultr_client).await?.unwrap();
                            let id = Uuid::from_str(&id).map_err(|_| ServiceError::NotFound)?;
                            let block_storage: BlockStorage = GetBlockStorage::new(id).execute(context.vultr_client).await?;
                            update_block_storage(&block_storage, trx).await?;
                        }
                        name if name.contains("AttachBlockStorageToCompute") => {
                            let command: AttachBlockStorageToCompute = serde_json::from_value(request.data)?;
                            command.execute(context.vultr_client).await?.unwrap();
                            let id = Uuid::from_str(&id).map_err(|_| ServiceError::NotFound)?;
                            let block_storage: BlockStorage = GetBlockStorage::new(id).execute(context.vultr_client).await?;
                            update_block_storage(&block_storage, trx).await?;
                        }
                        name if name.contains("DetachBlockStorageFromCompute") => {
                            let command: DetachBlockStorageFromCompute = serde_json::from_value(request.data)?;
                            command.execute(context.vultr_client).await?.unwrap();
                            let id = Uuid::from_str(&id).map_err(|_| ServiceError::NotFound)?;
                            let block_storage: BlockStorage = GetBlockStorage::new(id).execute(context.vultr_client).await?;
                            update_block_storage(&block_storage, trx).await?;
                        }
                        name if name.contains("UpdateFirewallGroup") => {
                            let command: UpdateFirewallGroup = serde_json::from_value(request.data)?;
                            command.execute(context.vultr_client).await?.unwrap();
                            let id = Uuid::from_str(&id).map_err(|_| ServiceError::NotFound)?;
                            let firewall_group: FirewallGroup = GetFirewallGroup::new(id).execute(context.vultr_client).await?;
                            update_firewall_group(&firewall_group, trx).await?;
                        }
                        name if name.contains("UpdateManagedDatabase") => {
                            let command: UpdateManagedDatabase = serde_json::from_value(request.data)?;
                            let res = command.execute(context.vultr_client).await?.unwrap();
                            let managed_database: ManagedDatabase = serde_json::from_value(res.clone())?;
                            update_managed_database(&managed_database, trx).await?;
                        }
                        name if name.contains("UpdateObjectStorage") => {
                            let command: UpdateObjectStorage = serde_json::from_value(request.data)?;
                            command.execute(context.vultr_client).await?.unwrap();
                            let id = Uuid::from_str(&id).map_err(|_| ServiceError::NotFound)?;
                            let object_storage: ObjectStorage = GetObjectStorage::new(id).execute(context.vultr_client).await?;
                            update_object_storage(&object_storage, trx).await?;
                        }
                        _ => return Err(ServiceError::NotFound),
                    }
                }
                name if name.contains("Delete") => {
                    let id = context.get_id_with_temp_id(&request.temp_id)?;
                    match request.command_name.as_str() {
                        name if name.contains("DeleteCompute") => {
                            let command: DeleteCompute = serde_json::from_value(request.data)?;
                            command.execute(context.vultr_client).await?;
                            let id = Uuid::from_str(&id).map_err(|_| ServiceError::NotFound)?;
                            delete_compute(&context.project_id, &id, trx).await?;
                        }
                        name if name.contains("DeleteBlockStorage") => {
                            let command: DeleteBlockStorage = serde_json::from_value(request.data)?;
                            command.execute(context.vultr_client).await?;
                            let id = Uuid::from_str(&id).map_err(|_| ServiceError::NotFound)?;
                            delete_block_storage(&context.project_id, &id, trx).await?;
                        }
                        name if name.contains("DeleteFirewallGroup") => {
                            let command: DeleteFirewallGroup = serde_json::from_value(request.data)?;
                            command.execute(context.vultr_client).await?;
                            let id = Uuid::from_str(&id).map_err(|_| ServiceError::NotFound)?;
                            delete_firewall_group(&context.project_id, &id, trx).await?;
                        }
                        name if name.contains("DeleteFirewallRule") => {
                            let command: DeleteFirewallRule = serde_json::from_value(request.data)?;
                            command.execute(context.vultr_client).await?;
                            let id = i64::from_str(&id).map_err(|_| ServiceError::NotFound)?;
                            delete_firewall_rule(&context.project_id, &id, trx).await?;
                        }
                        name if name.contains("DeleteManagedDatabase") => {
                            let command: DeleteManagedDatabase = serde_json::from_value(request.data)?;
                            command.execute(context.vultr_client).await?;
                            let id = Uuid::from_str(&id).map_err(|_| ServiceError::NotFound)?;
                            delete_managed_database(&context.project_id, &id, trx).await?;
                        }
                        name if name.contains("DeleteObjectStorage") => {
                            let command: DeleteObjectStorage = serde_json::from_value(request.data)?;
                            command.execute(context.vultr_client).await?;
                            let id = Uuid::from_str(&id).map_err(|_| ServiceError::NotFound)?;
                            delete_object_storage(&context.project_id, &id, trx).await?;
                        }
                        _ => return Err(ServiceError::NotFound),
                    }
                }
                _ => return Err(ServiceError::NotFound),
            }
        }
        Ok(())
    }
}

#[derive(Serialize, Deserialize, ToSchema)]
pub struct DeployProject {
    pub project_id: Uuid,
    pub command_list: CommandList,
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
    fn test_command_list_parsing() {
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
                        "region": "seoul",
                        "plan": "vultr-100",
                        "label": "test-compute",
                        "os_id": 123,
                        "backups": "enabled",
                        "hostname": "test-compute"
                    }
                },
                {
                    "command_name": "CreateBlockStorage",
                    "temp_id": "temp2",
                    "position": {
                        "x": 40,
                        "y": 40
                    },
                    "data": {
                        "region": "seoul",
                        "size_gb": 100,
                        "label": "test-storage"
                    }
                },
                {
                    "command_name": "CreateFirewallGroup",
                    "temp_id": "temp3",
                    "position": {
                        "x": 60,
                        "y": 60
                    },
                    "data": {
                        "description": "test firewall group",
                        "region": "seoul"
                    }
                },
                {
                    "command_name": "CreateFirewallRule",
                    "temp_id": "temp4",
                    "position": {
                        "x": 80,
                        "y": 80
                    },
                    "data": {
                        "firewall_group_id": "group1",
                        "protocol": "tcp",
                        "subnet": "0.0.0.0",
                        "subnet_size": 0,
                        "port": "80",
                        "notes": "test rule"
                    }
                },
                {
                    "command_name": "CreateManagedDatabase",
                    "temp_id": "temp5",
                    "position": {
                        "x": 100,
                        "y": 100
                    },
                    "data": {
                        "database_engine": "mysql",
                        "database_engine_version": "8",
                        "region": "seoul",
                        "plan": "vultr-dbaas-startup-cc-2-80-2",
                        "label": "test-db"
                    }
                },
                {
                    "command_name": "CreateObjectStorage",
                    "temp_id": "temp6",
                    "position": {
                        "x": 120,
                        "y": 120
                    },
                    "data": {
                        "cluster_id": 1,
                        "label": "test-object-storage"
                    }
                },
                {
                    "command_name": "UpdateCompute",
                    "temp_id": "temp1",
                    "position": {
                        "x": 20,
                        "y": 20
                    },
                    "data": {
                        "id": "temp1",
                        "backups": "enabled",
                        "firewall_group_id": "group1",
                        "os_id": 123,
                        "plan": "vultr-100",
                        "ddos_protection": true,
                        "label": "updated-compute"
                    }
                },
                {
                    "command_name": "UpdateBlockStorage",
                    "temp_id": "temp2",
                    "position": {
                        "x": 40,
                        "y": 40
                    },
                    "data": {
                        "id": "temp2",
                        "label": "updated-storage",
                        "size_gb": 200
                    }
                },
                {
                    "command_name": "AttachBlockStorageToCompute",
                    "temp_id": "temp2",
                    "position": {
                        "x": 40,
                        "y": 40
                    },
                    "data": {
                        "id": "temp2",
                        "instance_id": "temp1"
                    }
                },
                {
                    "command_name": "DetachBlockStorageFromCompute",
                    "temp_id": "temp2",
                    "position": {
                        "x": 40,
                        "y": 40
                    },
                    "data": {
                        "id": "temp2"
                    }
                },
                {
                    "command_name": "UpdateFirewallGroup",
                    "temp_id": "temp3",
                    "position": {
                        "x": 60,
                        "y": 60
                    },
                    "data": {
                        "id": "temp3",
                        "description": "updated firewall group"
                    }
                },
                {
                    "command_name": "UpdateManagedDatabase",
                    "temp_id": "temp5",
                    "position": {
                        "x": 100,
                        "y": 100
                    },
                    "data": {
                        "id": "temp5",
                        "label": "updated-db",
                        "database_engine_version": "8.0"
                    }
                },
                {
                    "command_name": "UpdateObjectStorage",
                    "temp_id": "temp6",
                    "position": {
                        "x": 120,
                        "y": 120
                    },
                    "data": {
                        "id": "temp6",
                        "label": "updated-object-storage"
                    }
                },
                {
                    "command_name": "DeleteCompute",
                    "temp_id": "temp1",
                    "position": {
                        "x": 20,
                        "y": 20
                    },
                    "data": {
                        "id": "temp1"
                    }
                },
                {
                    "command_name": "DeleteBlockStorage",
                    "temp_id": "temp2",
                    "position": {
                        "x": 40,
                        "y": 40
                    },
                    "data": {
                        "id": "temp2"
                    }
                },
                {
                    "command_name": "DeleteFirewallGroup",
                    "temp_id": "temp3",
                    "position": {
                        "x": 60,
                        "y": 60
                    },
                    "data": {
                        "id": "temp3"
                    }
                },
                {
                    "command_name": "DeleteFirewallRule",
                    "temp_id": "temp4",
                    "position": {
                        "x": 80,
                        "y": 80
                    },
                    "data": {
                        "firewall_rule_id": "rule1"
                    }
                },
                {
                    "command_name": "DeleteManagedDatabase",
                    "temp_id": "temp5",
                    "position": {
                        "x": 100,
                        "y": 100
                    },
                    "data": {
                        "id": "temp5"
                    }
                },
                {
                    "command_name": "DeleteObjectStorage",
                    "temp_id": "temp6",
                    "position": {
                        "x": 120,
                        "y": 120
                    },
                    "data": {
                        "id": "temp6"
                    }
                },
                {
                    "temp_id": "compute-1",
                    "command_name": "CreateInstance",
                    "position": {
                        "x": 500,
                        "y": 250
                    },
                    "data": {
                        "id": "",
                        "region": "ewr",
                        "plan": "vc2-2c-2gb",
                        "label": "Shopify-Web-Server",
                        "os_id": "2571",
                        "backups": "enable",
                        "hostname": "mislav.abha@example.com"
                    }
                },
                {
                    "temp_id": "objectstorage-1",
                    "command_name": "CreateObjectStorage",
                    "position": {
                        "x": 350,
                        "y": 400
                    },
                    "data": {
                        "id": "",
                        "cluster_id": "2",
                        "tier_id": "2",
                        "label": "Shopify-Asset-Storage"
                    }
                },
                {
                    "temp_id": "blockstorage-1",
                    "command_name": "CreateBlockStorage",
                    "position": {
                        "x": 500,
                        "y": 400
                    },
                    "data": {
                        "id": "",
                        "region": "ewr",
                        "label": "Shopify-Data-Volume"
                    }
                },
                {
                    "temp_id": "blockstorage-1",
                    "command_name": "AttachBlockStorageToInstance",
                    "position": {
                        "x": 500,
                        "y": 400
                    },
                    "data": {
                        "id": "",
                        "instance_id": "compute-1",
                        "live": true
                    }
                },
                {
                    "temp_id": "database-1",
                    "command_name": "CreateManagedDatabase",
                    "position": {
                        "x": 650,
                        "y": 400
                    },
                    "data": {
                        "id": "",
                        "database_engine": "pg",
                        "database_engine_version": "15",
                        "region": "ewr",
                        "plan": "vultr-dbaas-hobbyist-cc-1-25-1",
                        "label": "Shopify-PostgreSQL-DB"
                    }
                },
                {
                    "temp_id": "firewall-1",
                    "command_name": "CreateFirewallGroup",
                    "position": {
                        "x": 500,
                        "y": 100
                    },
                    "data": {
                        "id": "",
                        "description": "Allow HTTP"
                    }
                },
                {
                    "temp_id": "firewall-rule-1",
                    "command_name": "CreateFirewallRule",
                    "position": {
                        "x": 500,
                        "y": 400
                    },
                    "data": {
                        "firewall_group_id": "firewall-1",
                        "ip_type": "v4",
                        "protocol": "tcp",
                        "port": "80",
                        "subnet_size": 0,
                        "notes": "Public HTTP access"
                    }
                }        
            ]
        });

        let command_list: CommandList = serde_json::from_value(json).expect("Failed to parse CommandList");
        
        // Create 명령어들 검증
        let create_commands = &command_list.command_list[0..6];
        verify_create_compute(&create_commands[0]);
        verify_create_block_storage(&create_commands[1]);
        verify_create_firewall_group(&create_commands[2]);
        verify_create_firewall_rule(&create_commands[3]);
        verify_create_managed_database(&create_commands[4]);
        verify_create_object_storage(&create_commands[5]);

        // Update 명령어들 검증
        let update_commands = &command_list.command_list[6..13];
        verify_update_compute(&update_commands[0]);
        verify_update_block_storage(&update_commands[1]);
        verify_attach_block_storage(&update_commands[2]);
        verify_detach_block_storage(&update_commands[3]);
        verify_update_firewall_group(&update_commands[4]);
        verify_update_managed_database(&update_commands[5]);
        verify_update_object_storage(&update_commands[6]);

        // Delete 명령어들 검증
        let delete_commands = &command_list.command_list[13..19];
        verify_delete_compute(&delete_commands[0]);
        verify_delete_block_storage(&delete_commands[1]);
        verify_delete_firewall_group(&delete_commands[2]);
        verify_delete_firewall_rule(&delete_commands[3]);
        verify_delete_managed_database(&delete_commands[4]);
        verify_delete_object_storage(&delete_commands[5]);
    }

    fn verify_create_compute(command: &CommandRequest) {
        assert_eq!(command.command_name, "CreateCompute");
        assert_eq!(command.temp_id, "temp1");
        assert_eq!(command.position.x, 20);
        assert_eq!(command.position.y, 20);
        assert_eq!(command.data["region"], "seoul");
        assert_eq!(command.data["plan"], "vultr-100");
        assert_eq!(command.data["label"], "test-compute");
        assert_eq!(command.data["os_id"], 123);
        assert_eq!(command.data["backups"], "enabled");
        assert_eq!(command.data["hostname"], "test-compute");
    }

    fn verify_create_block_storage(command: &CommandRequest) {
        assert_eq!(command.command_name, "CreateBlockStorage");
        assert_eq!(command.temp_id, "temp2");
        assert_eq!(command.position.x, 40);
        assert_eq!(command.position.y, 40);
        assert_eq!(command.data["region"], "seoul");
        assert_eq!(command.data["size_gb"], 100);
        assert_eq!(command.data["label"], "test-storage");
    }

    fn verify_create_firewall_group(command: &CommandRequest) {
        assert_eq!(command.command_name, "CreateFirewallGroup");
        assert_eq!(command.temp_id, "temp3");
        assert_eq!(command.position.x, 60);
        assert_eq!(command.position.y, 60);
        assert_eq!(command.data["description"], "test firewall group");
        assert_eq!(command.data["region"], "seoul");
    }

    fn verify_create_firewall_rule(command: &CommandRequest) {
        assert_eq!(command.command_name, "CreateFirewallRule");
        assert_eq!(command.temp_id, "temp4");
        assert_eq!(command.position.x, 80);
        assert_eq!(command.position.y, 80);
        assert_eq!(command.data["firewall_group_id"], "group1");
        assert_eq!(command.data["protocol"], "tcp");
        assert_eq!(command.data["subnet"], "0.0.0.0");
        assert_eq!(command.data["subnet_size"], 0);
        assert_eq!(command.data["port"], "80");
        assert_eq!(command.data["notes"], "test rule");
    }

    fn verify_create_managed_database(command: &CommandRequest) {
        assert_eq!(command.command_name, "CreateManagedDatabase");
        assert_eq!(command.temp_id, "temp5");
        assert_eq!(command.position.x, 100);
        assert_eq!(command.position.y, 100);
        assert_eq!(command.data["database_engine"], "mysql");
        assert_eq!(command.data["database_engine_version"], "8");
        assert_eq!(command.data["region"], "seoul");
        assert_eq!(command.data["plan"], "vultr-dbaas-startup-cc-2-80-2");
        assert_eq!(command.data["label"], "test-db");
    }

    fn verify_create_object_storage(command: &CommandRequest) {
        assert_eq!(command.command_name, "CreateObjectStorage");
        assert_eq!(command.temp_id, "temp6");
        assert_eq!(command.position.x, 120);
        assert_eq!(command.position.y, 120);
        assert_eq!(command.data["cluster_id"], 1);
        assert_eq!(command.data["label"], "test-object-storage");
    }

    fn verify_update_compute(command: &CommandRequest) {
        assert_eq!(command.command_name, "UpdateCompute");
        assert_eq!(command.temp_id, "temp1");
        assert_eq!(command.position.x, 20);
        assert_eq!(command.position.y, 20);
        assert_eq!(command.data["id"], "temp1");
        assert_eq!(command.data["backups"], "enabled");
        assert_eq!(command.data["firewall_group_id"], "group1");
        assert_eq!(command.data["os_id"], 123);
        assert_eq!(command.data["plan"], "vultr-100");
        assert_eq!(command.data["ddos_protection"], true);
        assert_eq!(command.data["label"], "updated-compute");
    }

    fn verify_update_block_storage(command: &CommandRequest) {
        assert_eq!(command.command_name, "UpdateBlockStorage");
        assert_eq!(command.temp_id, "temp2");
        assert_eq!(command.position.x, 40);
        assert_eq!(command.position.y, 40);
        assert_eq!(command.data["id"], "temp2");
        assert_eq!(command.data["label"], "updated-storage");
        assert_eq!(command.data["size_gb"], 200);
    }

    fn verify_attach_block_storage(command: &CommandRequest) {
        assert_eq!(command.command_name, "AttachBlockStorageToCompute");
        assert_eq!(command.temp_id, "temp2");
        assert_eq!(command.position.x, 40);
        assert_eq!(command.position.y, 40);
        assert_eq!(command.data["id"], "temp2");
        assert_eq!(command.data["instance_id"], "temp1");
    }

    fn verify_detach_block_storage(command: &CommandRequest) {
        assert_eq!(command.command_name, "DetachBlockStorageFromCompute");
        assert_eq!(command.temp_id, "temp2");
        assert_eq!(command.position.x, 40);
        assert_eq!(command.position.y, 40);
        assert_eq!(command.data["id"], "temp2");
    }

    fn verify_update_firewall_group(command: &CommandRequest) {
        assert_eq!(command.command_name, "UpdateFirewallGroup");
        assert_eq!(command.temp_id, "temp3");
        assert_eq!(command.position.x, 60);
        assert_eq!(command.position.y, 60);
        assert_eq!(command.data["id"], "temp3");
        assert_eq!(command.data["description"], "updated firewall group");
    }

    fn verify_update_managed_database(command: &CommandRequest) {
        assert_eq!(command.command_name, "UpdateManagedDatabase");
        assert_eq!(command.temp_id, "temp5");
        assert_eq!(command.position.x, 100);
        assert_eq!(command.position.y, 100);
        assert_eq!(command.data["id"], "temp5");
        assert_eq!(command.data["label"], "updated-db");
        assert_eq!(command.data["database_engine_version"], "8.0");
    }

    fn verify_update_object_storage(command: &CommandRequest) {
        assert_eq!(command.command_name, "UpdateObjectStorage");
        assert_eq!(command.temp_id, "temp6");
        assert_eq!(command.position.x, 120);
        assert_eq!(command.position.y, 120);
        assert_eq!(command.data["id"], "temp6");
        assert_eq!(command.data["label"], "updated-object-storage");
    }

    fn verify_delete_compute(command: &CommandRequest) {
        assert_eq!(command.command_name, "DeleteCompute");
        assert_eq!(command.temp_id, "temp1");
        assert_eq!(command.position.x, 20);
        assert_eq!(command.position.y, 20);
        assert_eq!(command.data["id"], "temp1");
    }

    fn verify_delete_block_storage(command: &CommandRequest) {
        assert_eq!(command.command_name, "DeleteBlockStorage");
        assert_eq!(command.temp_id, "temp2");
        assert_eq!(command.position.x, 40);
        assert_eq!(command.position.y, 40);
        assert_eq!(command.data["id"], "temp2");
    }

    fn verify_delete_firewall_group(command: &CommandRequest) {
        assert_eq!(command.command_name, "DeleteFirewallGroup");
        assert_eq!(command.temp_id, "temp3");
        assert_eq!(command.position.x, 60);
        assert_eq!(command.position.y, 60);
        assert_eq!(command.data["id"], "temp3");
    }

    fn verify_delete_firewall_rule(command: &CommandRequest) {
        assert_eq!(command.command_name, "DeleteFirewallRule");
        assert_eq!(command.temp_id, "temp4");
        assert_eq!(command.position.x, 80);
        assert_eq!(command.position.y, 80);
        assert_eq!(command.data["firewall_rule_id"], "rule1");
    }

    fn verify_delete_managed_database(command: &CommandRequest) {
        assert_eq!(command.command_name, "DeleteManagedDatabase");
        assert_eq!(command.temp_id, "temp5");
        assert_eq!(command.position.x, 100);
        assert_eq!(command.position.y, 100);
        assert_eq!(command.data["id"], "temp5");
    }

    fn verify_delete_object_storage(command: &CommandRequest) {
        assert_eq!(command.command_name, "DeleteObjectStorage");
        assert_eq!(command.temp_id, "temp6");
        assert_eq!(command.position.x, 120);
        assert_eq!(command.position.y, 120);
        assert_eq!(command.data["id"], "temp6");
    }
}

