-- Add up migration script here
CREATE TABLE Webhook (
  id SERIAL PRIMARY KEY,
  listening_account VARCHAR(255) DEFAULT NULL,
  url VARCHAR(512) NOT NULL,
  CONSTRAINT fk_listening_account 
    FOREIGN KEY(listening_account) REFERENCES Account(username) ON DELETE SET NULL
);
