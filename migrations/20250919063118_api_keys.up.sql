-- Add up migration script here
CREATE TABLE api_keys (
  id SERIAL PRIMARY KEY,
  key VARCHAR(255) NOT NULL UNIQUE,
  is_active BOOLEAN DEFAULT TRUE
);

