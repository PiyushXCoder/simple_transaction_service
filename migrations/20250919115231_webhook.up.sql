-- Add up migration script here
CREATE TABLE Webhook (
  id SERIAL PRIMARY KEY,
  listening_account VARCHAR(255) NOT NULL,
  url VARCHAR(512) NOT NULL,
  CONSTRAINT fk_listening_account 
    FOREIGN KEY(listening_account) REFERENCES Account(username) ON DELETE SET NULL
);

CREATE TABLE WebhookQueue (
  id SERIAL PRIMARY KEY,
  url VARCHAR(512) NOT NULL,
  listening_account VARCHAR(255) NOT NULL,
  transaction_id INT NOT NULL, 
  event VARCHAR(255) NOT NULL,
  message TEXT NOT NULL,
  status VARCHAR(50) DEFAULT 'pending' NOT NULL
);
