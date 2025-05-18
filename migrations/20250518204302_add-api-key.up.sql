-- Add up migration script here

CREATE TABLE IF NOT EXISTS vult_api_key(
    project_id UUID NOT NULL UNIQUE,
    api_key VARCHAR(255) NOT NULL,
    update_dt TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    CONSTRAINT vult_api_key_project_id_fkey FOREIGN KEY (project_id) REFERENCES project(id) ON DELETE CASCADE
);
