CREATE TABLE users (
    pubkey text NOT NULL PRIMARY KEY,
    username text NOT NULL,
    created_at timestamptz NOT NULL
);
