CREATE TABLE IF NOT EXISTS users
(
    id           TEXT        PRIMARY KEY,
    username     TEXT        NOT NULL UNIQUE,
    password     TEXT        NOT NULL,
    user_roles   TEXT        NOT NULL,
    first_name   TEXT        NOT NULL,
    last_name    TEXT        NOT NULL,
    mobile_phone TEXT        NOT NULL,
    created_at   TIMESTAMPTZ NOT NULL DEFAULT NOW()
);
