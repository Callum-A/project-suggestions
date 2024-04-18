-- Add migration script here
-- #[derive(Debug)]
-- pub struct Tag {
--     pub id: i32,
--     pub public_id: String,
--     pub name: String,
-- }

-- #[derive(Debug)]
-- pub struct TagToProject {
--     pub tag_id: i32,
--     pub project_id: i32,
-- }


CREATE TABLE tag (
    id SERIAL PRIMARY KEY,
    public_id TEXT UNIQUE NOT NULL,
    name TEXT UNIQUE NOT NULL
);

CREATE INDEX tag_public_id_index ON tag(public_id);
CREATE INDEX tag_name_index ON tag(name);

CREATE TABLE tag_to_project (
    tag_id INT NOT NULL,
    project_id INT NOT NULL,
    PRIMARY KEY (tag_id, project_id),
    FOREIGN KEY (tag_id) REFERENCES tag(id) ON DELETE CASCADE,
    FOREIGN KEY (project_id) REFERENCES project(id) ON DELETE CASCADE
);

CREATE INDEX tag_to_project_tag_id_index ON tag_to_project(tag_id);
CREATE INDEX tag_to_project_project_id_index ON tag_to_project(project_id);