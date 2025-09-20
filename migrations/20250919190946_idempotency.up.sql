-- Add up migration script here
CREATE TABLE Idempotency (
    key TEXT PRIMARY KEY,
    status_code INTEGER NOT NULL,
    response BYTEA NOT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP NOT NULL
);
