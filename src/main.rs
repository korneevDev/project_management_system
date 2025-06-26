use actix_web::{web, App, HttpResponse, HttpServer};
use dotenv::dotenv;
use diesel::r2d2::{self, ConnectionManager};
use diesel::PgConnection;

mod models;
mod repositories;
mod schema;

use crate::repositories::Repository;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    
    let database_url = std::env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    let pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool");

    let jwt_secret = std::env::var("JWT_SECRET")
        .expect("JWT_SECRET must be set");
    let jwt_expiry_hours = std::env::var("JWT_EXPIRY_HOURS")
        .unwrap_or("24".to_string())
        .parse()
        .expect("JWT_EXPIRY_HOURS must be a number");

    let repo = Repository::new(pool.clone(), jwt_secret, jwt_expiry_hours);

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(repo.clone()))
            // Здесь добавьте ваши маршруты
            .route("/", web::get().to(|| async { HttpResponse::Ok().body("Hello world!") }))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}