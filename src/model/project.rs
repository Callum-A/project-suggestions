use chrono::NaiveDateTime;
use serde::Serialize;

#[derive(Debug)]
pub struct Project {
    pub id: i32,
    pub public_id: String,
    pub title: String,
    pub description: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub user_id: i32,
}

#[derive(Debug, Serialize)]
pub struct SerializableProject {
    pub id: i32,
    pub public_id: String,
    pub title: String,
    pub description: String,
    pub created_at: String,
    pub updated_at: String,
    pub user_id: i32,
}

impl From<Project> for SerializableProject {
    fn from(val: Project) -> Self {
        SerializableProject {
            id: val.id,
            public_id: val.public_id,
            title: val.title,
            description: val.description,
            created_at: val.created_at.to_string(),
            updated_at: val.updated_at.to_string(),
            user_id: val.user_id,
        }
    }
}
