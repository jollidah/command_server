-- Add down migration script here

DROP TABLE IF EXISTS object_position;
DROP TABLE IF EXISTS object_storage;
DROP TABLE IF EXISTS managed_database;
DROP TABLE IF EXISTS compute;
DROP TABLE IF EXISTS firewall_rule;
DROP TABLE IF EXISTS firewall_group;
DROP TABLE IF EXISTS block_storage;
DROP TYPE IF EXISTS auto_backups;
DROP TYPE IF EXISTS block_storage_type;
