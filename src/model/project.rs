use chrono::NaiveDateTime;

#[derive(Debug)]
pub struct Project {
    pub id: i32,
    pub public_id: String,
    pub title: String,
    pub description: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}
