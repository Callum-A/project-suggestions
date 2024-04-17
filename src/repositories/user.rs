use crate::model::user::{AuthProvider, User};
use sqlx::{Pool, Postgres};

#[derive(Debug, Clone)]
pub struct UserRepository {
    pub pool: Pool<Postgres>,
}

impl UserRepository {
    pub fn new(pool: Pool<Postgres>) -> Self {
        Self { pool }
    }

    pub async fn find_by_id(&self, id: i32) -> Option<User> {
        sqlx::query_as!(
            User,
            r#"
            SELECT *
            FROM "user"
            WHERE id = $1
            "#,
            id
        )
        .fetch_optional(&self.pool)
        .await
        .unwrap()
    }

    pub async fn find_by_email(&self, email: &str) -> Option<User> {
        sqlx::query_as!(
            User,
            r#"
            SELECT *
            FROM "user"
            WHERE email = $1
            "#,
            email
        )
        .fetch_optional(&self.pool)
        .await
        .unwrap()
    }

    pub async fn create(
        &self,
        public_id: &str,
        email: &str,
        name: &str,
        provider: AuthProvider,
    ) -> User {
        sqlx::query_as!(
            User,
            r#"
            INSERT INTO "user" (public_id, email, name, auth_provider)
            VALUES ($1, $2, $3, $4)
            RETURNING *
            "#,
            public_id,
            email,
            name,
            provider.as_str()
        )
        .fetch_one(&self.pool)
        .await
        .unwrap()
    }
}
