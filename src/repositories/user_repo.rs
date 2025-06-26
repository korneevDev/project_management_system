use diesel::prelude::*;
use super::DbPool;
use crate::models::{User, NewUser};
use crate::schema::users;

#[derive(Clone)]
pub struct UserRepository {
    pool: DbPool,
}

impl UserRepository {
    pub fn new(pool: DbPool) -> Self {
        Self { pool }
    }

    pub fn create_user(&self, email: &str, password_hash: &str, role: &str) -> Result<User, diesel::result::Error> {
        let new_user = NewUser {
            email: email.to_string(),
            password_hash: password_hash.to_string(),
            role: role.to_string(),
        };

        let mut conn = self.pool.get().unwrap();
        diesel::insert_into(users::table)
            .values(&new_user)
            .get_result(&mut conn)
    }

    pub fn find_by_email(&self, user_email: &str) -> Option<User> {
        use crate::schema::users::dsl::{users, email};
        
        let mut conn = self.pool.get().unwrap();
        users
            .filter(email.eq(user_email))
            .first(&mut conn)
            .optional()
            .unwrap_or(None)
    }
}