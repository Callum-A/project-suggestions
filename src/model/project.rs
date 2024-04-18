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

impl Into<SerializableProject> for Project {
    fn into(self) -> SerializableProject {
        SerializableProject {
            id: self.id,
            public_id: self.public_id,
            title: self.title,
            description: self.description,
            created_at: self.created_at.to_string(),
            updated_at: self.updated_at.to_string(),
            user_id: self.user_id,
        }
    }
}
