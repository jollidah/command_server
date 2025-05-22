-- Add up migration script here

CREATE TYPE auto_backups AS ENUM('enabled', 'disabled');
CREATE TYPE ip_type AS ENUM('v4', 'v6');
CREATE TYPE protocol AS ENUM('icmp', 'tcp', 'udp', 'gre', 'esp', 'ah');
CREATE TYPE database_engine AS ENUM('mysql', 'pg');
CREATE TYPE resource_type AS ENUM('block_storage', 'compute', 'managed_database', 'object_storage', 'firewall_group', 'firewall_rule');

CREATE TABLE IF NOT EXISTS block_storage (
    project_id UUID NOT NULL,
    y BIGINT NOT NULL,
    x BIGINT NOT NULL,
    region_id VARCHAR(255) NOT NULL,
    id UUID NOT NULL,
    mount_id VARCHAR(255) NOT NULL,
    attached_to_instance UUID NOT NULL,
    size_gb BIGINT NOT NULL,
    label VARCHAR(255) NOT NULL,
    CONSTRAINT block_storage_pkey PRIMARY KEY (project_id, id),
    CONSTRAINT block_storage_project_id_fkey FOREIGN KEY (project_id) REFERENCES project(id) ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS firewall_group (
    project_id UUID NOT NULL,
    y BIGINT NOT NULL,
    x BIGINT NOT NULL,
    id UUID NOT NULL,
    description TEXT NOT NULL,
    CONSTRAINT firewall_group_pkey PRIMARY KEY (project_id, id),
    CONSTRAINT firewall_group_project_id_fkey FOREIGN KEY (project_id) REFERENCES project(id) ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS firewall_rule (
    project_id UUID NOT NULL,
    y BIGINT NOT NULL,
    x BIGINT NOT NULL,
    id BIGINT NOT NULL,
    action VARCHAR(255) NOT NULL,
    port VARCHAR(255) NOT NULL,
    ip_type ip_type NOT NULL,
    protocol protocol NOT NULL,
    subnet VARCHAR(255) NOT NULL,
    subnet_size BIGINT NOT NULL,
    notes TEXT NOT NULL,
    CONSTRAINT firewall_rule_pkey PRIMARY KEY (project_id, id),
    CONSTRAINT firewall_rule_project_id_fkey FOREIGN KEY (project_id) REFERENCES project(id) ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS compute (
    project_id UUID NOT NULL,
    y BIGINT NOT NULL,
    x BIGINT NOT NULL,
    region_id VARCHAR(255) NOT NULL,
    id UUID NOT NULL,
    plan VARCHAR(255) NOT NULL,
    status VARCHAR(255) NOT NULL,
    main_ip VARCHAR(255) NOT NULL,
    label VARCHAR(255) NOT NULL,
    os_id BIGINT NOT NULL,
    firewall_group_id VARCHAR(255) NOT NULL,
    auto_backups auto_backups NOT NULL DEFAULT 'disabled',
    CONSTRAINT compute_pkey PRIMARY KEY (project_id, id),
    CONSTRAINT compute_project_id_fkey FOREIGN KEY (project_id) REFERENCES project(id) ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS managed_database (
    project_id UUID NOT NULL,
    y BIGINT NOT NULL,
    x BIGINT NOT NULL,
    region_id VARCHAR(255) NOT NULL,
    id UUID NOT NULL,
    status VARCHAR(255) NOT NULL,
    plan VARCHAR(255) NOT NULL,
    database_engine database_engine NOT NULL,
    database_engine_version BIGINT NOT NULL,
    latest_backup VARCHAR(255) NOT NULL,
    label VARCHAR(255) NOT NULL,
    CONSTRAINT managed_database_pkey PRIMARY KEY (project_id, id),
    CONSTRAINT managed_database_project_id_fkey FOREIGN KEY (project_id) REFERENCES project(id) ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS object_storage (
    project_id UUID NOT NULL,
    y BIGINT NOT NULL,
    x BIGINT NOT NULL,
    tier_id BIGINT NOT NULL,
    id UUID NOT NULL,
    cluster_id BIGINT NOT NULL,
    label VARCHAR(255) NOT NULL,
    CONSTRAINT object_storage_pkey PRIMARY KEY (project_id, id),
    CONSTRAINT object_storage_project_id_fkey FOREIGN KEY (project_id) REFERENCES project(id) ON DELETE CASCADE
);
