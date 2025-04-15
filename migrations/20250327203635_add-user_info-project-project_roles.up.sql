-- Add up migration script here
CREATE TYPE role AS ENUM('admin', 'editor', 'viewer');

CREATE TABLE IF NOT EXISTS account_user(
    id UUID PRIMARY KEY,
    email VARCHAR(255) NOT NULL,
    name VARCHAR(255) NOT NULL,
    phone_num VARCHAR(255) NOT NULL,
    password VARCHAR(255) NOT NULL,
    verified BOOLEAN NOT NULL DEFAULT FALSE,
    create_dt TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    UNIQUE(email)
);

CREATE TABLE IF NOT EXISTS project(
    id UUID PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    description TEXT NOT NULL,
    create_dt TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    update_dt TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    version BIGINT NOT NULL DEFAULT 1
);

CREATE TABLE IF NOT EXISTS user_role(
    project_id UUID NOT NULL,
    user_email VARCHAR(255) NOT NULL,
    role role NOT NULL,
    update_dt TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    CONSTRAINT user_role_pkey PRIMARY KEY (project_id, user_email)
);
