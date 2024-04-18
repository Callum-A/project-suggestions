use crate::model::tag::{Tag, TagToProject};
use sqlx::{Pool, Postgres};

#[derive(Debug, Clone)]
pub struct TagRepository {
    pub pool: Pool<Postgres>,
}

impl TagRepository {
    pub fn new(pool: Pool<Postgres>) -> Self {
        Self { pool }
    }

    pub async fn create(&self, name: &str, public_id: &str) -> Tag {
        sqlx::query_as!(
            Tag,
            r#"
            INSERT INTO "tag" (name, public_id)
            VALUES ($1, $2)
            RETURNING *
            "#,
            name,
            public_id
        )
        .fetch_one(&self.pool)
        .await
        .unwrap()
    }

    pub async fn find_by_name_or_create(&self, name: &str) -> Tag {
        match self.find_by_name(name).await {
            Some(tag) => tag,
            None => {
                let public_id = uuid::Uuid::new_v4().to_string();
                self.create(name, &public_id).await
            }
        }
    }

    pub async fn create_tag_to_project(&self, tag_id: i32, project_id: i32) -> TagToProject {
        sqlx::query_as!(
            TagToProject,
            r#"
            INSERT INTO "tag_to_project" (tag_id, project_id)
            VALUES ($1, $2)
            RETURNING *
            "#,
            tag_id,
            project_id
        )
        .fetch_one(&self.pool)
        .await
        .unwrap()
    }

    pub async fn delete_tag_to_project(&self, tag_id: i32, project_id: i32) {
        sqlx::query!(
            r#"
            DELETE FROM "tag_to_project"
            WHERE tag_id = $1 AND project_id = $2
            "#,
            tag_id,
            project_id
        )
        .execute(&self.pool)
        .await
        .unwrap();
    }

    pub async fn find_by_id(&self, id: i32) -> Option<Tag> {
        sqlx::query_as!(
            Tag,
            r#"
            SELECT *
            FROM "tag"
            WHERE id = $1
            "#,
            id
        )
        .fetch_optional(&self.pool)
        .await
        .unwrap()
    }

    pub async fn find_by_name(&self, name: &str) -> Option<Tag> {
        sqlx::query_as!(
            Tag,
            r#"
            SELECT *
            FROM "tag"
            WHERE name = $1
            "#,
            name
        )
        .fetch_optional(&self.pool)
        .await
        .unwrap()
    }

    pub async fn find_by_public_id(&self, public_id: &str) -> Option<Tag> {
        sqlx::query_as!(
            Tag,
            r#"
            SELECT *
            FROM "tag"
            WHERE public_id = $1
            "#,
            public_id
        )
        .fetch_optional(&self.pool)
        .await
        .unwrap()
    }

    pub async fn find_by_project_id(&self, project_id: i32) -> Vec<Tag> {
        sqlx::query_as!(
            Tag,
            r#"
            SELECT t.*
            FROM "tag" t
            JOIN "tag_to_project" tp ON t.id = tp.tag_id
            WHERE tp.project_id = $1
            "#,
            project_id
        )
        .fetch_all(&self.pool)
        .await
        .unwrap()
    }
}
