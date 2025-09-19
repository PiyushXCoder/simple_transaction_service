-- Add up migration script here

CREATE TABLE Account (
  username VARCHAR(255) PRIMARY KEY,
  name VARCHAR(255) NOT NULL,
  balance BIGINT NOT NULL DEFAULT 0 
);

CREATE TABLE Transaction (
  id SERIAL PRIMARY KEY,
  sender VARCHAR(255) DEFAULT NULL,
  receiver VARCHAR(255) NOT NULL,
  amount BIGINT NOT NULL,
  timestamp TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
  CONSTRAINT fk_sender 
    FOREIGN KEY(sender) REFERENCES Account(username) ON DELETE SET NULL,
  CONSTRAINT fk_receiver
    FOREIGN KEY(receiver) REFERENCES Account(username) ON DELETE SET NULL
);
