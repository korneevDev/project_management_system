use actix_web::{web, App, HttpServer};
use diesel::r2d2::{self, ConnectionManager};
use diesel::PgConnection;

mod models;
mod repositories;
mod schema;
mod auth;

use crate::repositories::Repository;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Настройка подключения к БД
    let manager = ConnectionManager::<PgConnection>::new(
        "postgres://postgres:root@localhost:5433/project_management_test"
    );
    let pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool");

    // Настройка JWT
    let jwt_secret = "your_very_secret_key".to_string();
    let jwt_expiry_hours = 24;

    let repo = Repository::new(pool, jwt_secret, jwt_expiry_hours);

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(repo.clone()))
            .service(auth::login)
            .service(auth::refresh)
            .service(auth::logout)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}