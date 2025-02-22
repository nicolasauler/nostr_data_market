CREATE TABLE mqtt_raw (
    id int PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
    time timestamptz,
    topic text,
    payload jsonb
);
