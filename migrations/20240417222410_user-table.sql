-- Add migration script here
CREATE TABLE "user" (
    id SERIAL PRIMARY KEY,
    public_id TEXT UNIQUE NOT NULL,
    name TEXT NOT NULL,
    email TEXT UNIQUE NOT NULL,
    auth_provider TEXT NOT NULL
);

CREATE INDEX user_email_index ON "user"(email);
CREATE INDEX user_public_id_index ON "user"(public_id);