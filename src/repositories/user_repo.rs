use crate::{models::User, schema::users};
use diesel::prelude::*;
use super::DbPool;

pub struct UserRepository {
    pool: DbPool,
}

impl UserRepository {
    pub fn new(pool: DbPool) -> Self {
        Self { pool }
    }

    pub fn create_user(&self, email: &str, password_hash: &str, role: &str) -> Result<User, diesel::result::Error> {
        use crate::schema::users::dsl::*;

        let mut conn = self.pool.get().unwrap();
        diesel::insert_into(users)
            .values((
                email.eq(email),
                password_hash.eq(password_hash),
                role.eq(role),
            ))
            .get_result(&mut conn)
    }

    pub fn find_by_email(&self, user_email: &str) -> Option<User> {
        use crate::schema::users::dsl::*;

        let mut conn = self.pool.get().unwrap();
        users
            .filter(email.eq(user_email))
            .first(&mut conn)
            .optional()
            .unwrap()
    }

    pub fn get_user_role(&self, user_id: i32) -> Option<String> {
        use crate::schema::users::dsl::*;

        let mut conn = self.pool.get().unwrap();
        users
            .filter(id.eq(user_id))
            .select(role)
            .first(&mut conn)
            .optional()
            .unwrap()
    }
}