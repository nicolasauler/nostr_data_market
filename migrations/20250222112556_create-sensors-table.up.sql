CREATE TABLE user_sensors (
  id INT PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
  user_pubkey TEXT NOT NULL REFERENCES users (pubkey),
  external_id TEXT NOT NULL,
  description TEXT NOT NULL,
  created_at TIMESTAMPTZ NOT NULL
);
