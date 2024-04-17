-- Add migration script here
-- pub struct Project {
--     pub id: i32,
--     pub public_id: String,
--     pub title: String,
--     pub description: String,
--     pub created_at: NaiveDateTime,
--     pub updated_at: NaiveDateTime,
-- }

CREATE TABLE project (
    id SERIAL PRIMARY KEY,
    public_id TEXT UNIQUE NOT NULL,
    title TEXT NOT NULL,
    description TEXT NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX project_public_id_index ON project(public_id);
CREATE INDEX project_title_index ON project(title);
CREATE INDEX project_created_at_index ON project(created_at);
CREATE INDEX project_updated_at_index ON project(updated_at);