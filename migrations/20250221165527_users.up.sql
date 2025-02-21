CREATE TABLE users (
    id integer PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
    username text NOT NULL,
    created_at timestamptz NOT NULL,
    pubkey text NOT NULL UNIQUE
);

