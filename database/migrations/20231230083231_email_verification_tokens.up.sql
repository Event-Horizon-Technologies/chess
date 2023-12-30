ALTER TABLE accounts ADD COLUMN email_verified BOOLEAN NOT NULL DEFAULT FALSE;

CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

CREATE TABLE email_verification_tokens (
    token UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    account_id INTEGER NOT NULL REFERENCES accounts(id),
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    expires_at TIMESTAMP NOT NULL,
    used BOOLEAN NOT NULL DEFAULT FALSE
);