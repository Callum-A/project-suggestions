use crate::model::project::Project;
use sqlx::{Pool, Postgres};

#[derive(Debug, Clone)]
pub struct ProjectRepository {
    pub pool: Pool<Postgres>,
}

impl ProjectRepository {
    pub fn new(pool: Pool<Postgres>) -> Self {
        Self { pool }
    }

    pub async fn find_by_id(&self, id: i32) -> Option<Project> {
        sqlx::query_as!(
            Project,
            r#"
            SELECT *
            FROM "project"
            WHERE id = $1
            "#,
            id
        )
        .fetch_optional(&self.pool)
        .await
        .unwrap()
    }
}
