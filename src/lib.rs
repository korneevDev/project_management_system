use diesel::{PgConnection, Connection};  // Добавлен импорт Connection
use diesel::r2d2::{ConnectionManager, Pool};
use dotenv::dotenv;
use std::env;

pub type DbPool = Pool<ConnectionManager<PgConnection>>;

pub fn establish_test_connection() -> PgConnection {
    dotenv().ok();
    let database_url = env::var("TEST_DATABASE_URL")
        .unwrap_or("postgres://postgres:root@localhost:5433/project_management_test".into());
    
    // Теперь метод establish будет доступен
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

pub mod models;
pub mod repositories;
pub mod schema;