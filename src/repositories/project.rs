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

    pub async fn paginate(&self, page: i64, limit: i64) -> Vec<Project> {
        sqlx::query_as!(
            Project,
            r#"
            SELECT *
            FROM "project"
            ORDER BY created_at DESC
            LIMIT $1 OFFSET $2
            "#,
            limit,
            (page - 1) * limit
        )
        .fetch_all(&self.pool)
        .await
        .unwrap()
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

    pub async fn find_by_public_id(&self, public_id: &str) -> Option<Project> {
        sqlx::query_as!(
            Project,
            r#"
            SELECT *
            FROM "project"
            WHERE public_id = $1
            "#,
            public_id
        )
        .fetch_optional(&self.pool)
        .await
        .unwrap()
    }

    pub async fn create(&self, public_id: &str, title: &str, description: &str) -> Project {
        sqlx::query_as!(
            Project,
            r#"
            INSERT INTO "project" (public_id, title, description)
            VALUES ($1, $2, $3)
            RETURNING *
            "#,
            public_id,
            title,
            description
        )
        .fetch_one(&self.pool)
        .await
        .unwrap()
    }

    pub async fn update(&self, project: &Project) -> Project {
        sqlx::query_as!(
            Project,
            r#"
            UPDATE "project"
            SET title = $1, description = $2,
            updated_at = CURRENT_TIMESTAMP
            WHERE id = $3
            RETURNING *
            "#,
            project.title,
            project.description,
            project.id
        )
        .fetch_one(&self.pool)
        .await
        .unwrap()
    }

    pub async fn delete_by_id(&self, id: i32) {
        sqlx::query_as!(
            Project,
            r#"
            DELETE FROM "project"
            WHERE id = $1
            "#,
            id
        )
        .execute(&self.pool)
        .await
        .unwrap();
    }

    pub async fn delete_by_public_id(&self, public_id: &str) {
        sqlx::query_as!(
            Project,
            r#"
            DELETE FROM "project"
            WHERE public_id = $1
            "#,
            public_id
        )
        .execute(&self.pool)
        .await
        .unwrap();
    }
}
