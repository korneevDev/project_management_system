use chrono::Utc;
use diesel::prelude::*;
use super::DbPool;
use crate::models::UserSession; // Добавляем импорт UserSession
use crate::models::NewUserSession;
#[derive(Clone)]
pub struct AuthRepository {
    pub pool: DbPool,
    pub jwt_secret: String,
    pub jwt_expiry_hours: i64,
}

impl AuthRepository {
    pub fn new(pool: DbPool, jwt_secret: String, jwt_expiry_hours: i64) -> Self {
        Self { pool, jwt_secret, jwt_expiry_hours }
    }

    pub fn create_session(&self, user_id: i32, token: &str) -> Result<(), diesel::result::Error> {
        use crate::schema::user_sessions::dsl;

        let new_session = NewUserSession {
            user_id,
            token: token.to_string(),
            expires_at: Utc::now().naive_utc() + chrono::Duration::hours(self.jwt_expiry_hours),
        };

        diesel::insert_into(dsl::user_sessions)
            .values(&new_session)
            .execute(&mut self.pool.get().unwrap())?;

        Ok(())
    }

    pub fn find_session(&self, session_token: &str) -> Option<UserSession> {
        use crate::schema::user_sessions::dsl::*;

        user_sessions
            .filter(token.eq(session_token))
            .first::<UserSession>(&mut self.pool.get().unwrap())
            .optional()
            .unwrap()
    }

    pub fn update_session(&self, session_id: i32, new_token: &str) -> Result<(), diesel::result::Error> {
        use crate::schema::user_sessions::dsl::*;

        let new_expires_at = Utc::now() + chrono::Duration::hours(self.jwt_expiry_hours);
        let expires_at_value = new_expires_at.naive_utc();

        diesel::update(user_sessions.find(session_id))
            .set((
                token.eq(new_token),
                expires_at.eq(expires_at_value),
            ))
            .execute(&mut self.pool.get().unwrap())?;

        Ok(())
    }

    pub fn delete_session_by_token(&self, session_token: &str) -> Result<(), diesel::result::Error> {
        use crate::schema::user_sessions::dsl::*;

        diesel::delete(user_sessions.filter(token.eq(session_token)))
            .execute(&mut self.pool.get().unwrap())?;

        Ok(())
    }
}