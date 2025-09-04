use std::error::Error;
use std::sync::Arc;

use sqlx::PgPool;

use crate::config::crypto::CryptoService;
use crate::models::user::{NewUser, User};

pub struct UserRepository {
    pool: Arc<PgPool>,
}

impl UserRepository {
    pub fn new(pool: Arc<PgPool>) -> Self {
        Self { pool }
    }

    pub async fn create(&self, new_user: NewUser, crypto_service: &CryptoService) -> Result<User, Box<dyn Error>> {
        let password_hash = crypto_service.hash_password(new_user.password).await?;

        let user = sqlx::query_as::<_, User>(
            "INSERT INTO users (username, email, password_hash) VALUES ($1, $2, $3) RETURNING id, email, created_at"
            )
            .bind(new_user.username)
            .bind(new_user.email)
            .bind(password_hash)
            .fetch_one(&*self.pool)
            .await?;
        
        Ok(user)
    }
} 

