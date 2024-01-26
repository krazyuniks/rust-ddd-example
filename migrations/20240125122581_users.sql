CREATE TABLE IF NOT EXISTS users
(
    id          TEXT        PRIMARY KEY,
    username    TEXT        NOT NULL UNIQUE,
    password    TEXT        NOT NULL,
    user_roles  TEXT        NOT NULL,
    created_at  TIMESTAMPTZ NOT NULL DEFAULT NOW()
);
