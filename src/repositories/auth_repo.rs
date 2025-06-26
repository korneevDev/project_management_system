use chrono::{Duration, Utc};
use diesel::prelude::*;
use jsonwebtoken::{EncodingKey, Header};
use serde::{Deserialize, Serialize};

use crate::models::UserSession;
use crate::schema::user_sessions;
use super::DbPool;

#[derive(Debug, Serialize, Deserialize)]
pub struct JwtClaims {
    pub sub: i32,       // user_id
    pub exp: usize,     // expiry timestamp
    pub role: String,   // user role
}

#[derive(Clone)]
pub struct AuthRepository {
    pool: DbPool,
    jwt_secret: String,
    jwt_expiry_hours: i64,
}

impl AuthRepository {
    pub fn new(pool: DbPool, jwt_secret: String, jwt_expiry_hours: i64) -> Self {
        Self {
            pool,
            jwt_secret,
            jwt_expiry_hours,
        }
    }

    /// Генерация нового JWT токена
    pub fn generate_token(&self, user_id: i32, role: &str) -> String {
        let expiration = Utc::now()
            .checked_add_signed(Duration::hours(self.jwt_expiry_hours))
            .expect("Invalid timestamp")
            .timestamp() as usize;

        let claims = JwtClaims {
            sub: user_id,
            exp: expiration,
            role: role.to_string(),
        };

        jsonwebtoken::encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(self.jwt_secret.as_ref()),
        )
        .unwrap()
    }

    /// Сохранение сессии в БД
    pub fn create_session(&self, user_id: i32, token: &str) -> Result<UserSession, diesel::result::Error> {
        let expires_at = Utc::now()
            .checked_add_signed(Duration::hours(self.jwt_expiry_hours))
            .unwrap();

        let mut conn = self.pool.get().unwrap();
        diesel::insert_into(user_sessions::table)
            .values((
                user_sessions::user_id.eq(user_id),
                user_sessions::token.eq(token),
                user_sessions::expires_at.eq(expires_at.naive_utc()),
            ))
            .get_result(&mut conn)
    }

    /// Проверка валидности токена
    pub fn validate_token(&self, token: &str) -> Option<JwtClaims> {
        use jsonwebtoken::{decode, DecodingKey, Validation};

        decode::<JwtClaims>(
            token,
            &DecodingKey::from_secret(self.jwt_secret.as_ref()),
            &Validation::default(),
        )
        .map(|data| data.claims)
        .ok()
    }

    /// Поиск активной сессии
    pub fn find_session(&self, _token: &str) -> Option<UserSession> {
        use crate::schema::user_sessions::dsl::*;
        let mut conn = self.pool.get().unwrap();

        user_sessions
            .filter(token.eq(token))
            .filter(expires_at.gt(Utc::now().naive_utc()))
            .first(&mut conn)
            .optional()
            .unwrap()
    }

    /// Удаление всех просроченных сессий
    pub fn cleanup_expired_sessions(&self) -> usize {
        use crate::schema::user_sessions::dsl::*;
        let mut conn = self.pool.get().unwrap();

        diesel::delete(user_sessions.filter(expires_at.lt(Utc::now().naive_utc())))
            .execute(&mut conn)
            .unwrap()
    }
}