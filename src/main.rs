mod models;
mod schema;
mod repositories;

use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use repositories::{ProjectRepository, TaskRepository, UserRepository};
use diesel::{r2d2::ConnectionManager, PgConnection};
use dotenv::dotenv;
use std::env;

pub type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;

async fn index() -> impl Responder {
    HttpResponse::Ok().body("Project Management System")
}

async fn get_projects(
    pool: web::Data<DbPool>,
    project_repo: web::Data<ProjectRepository>,
) -> impl Responder {
    match project_repo.all() {
        Ok(projects) => HttpResponse::Ok().json(projects),
        Err(_) => HttpResponse::InternalServerError().body("Error getting projects"),
    }
}

async fn create_project(
    pool: web::Data<DbPool>,
    project_repo: web::Data<ProjectRepository>,
    new_project: web::Json<NewProject>,
) -> impl Responder {
    match project_repo.create(new_project.into_inner()) {
        Ok(project) => HttpResponse::Created().json(project),
        Err(_) => HttpResponse::InternalServerError().body("Error creating project"),
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    
    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    let pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create DB pool");
    
    let project_repo = web::Data::new(ProjectRepository::new(pool.clone()));
    let task_repo = web::Data::new(TaskRepository::new(pool.clone()));
    let user_repo = web::Data::new(UserRepository::new(pool.clone()));

    HttpServer::new(move || {
        App::new()
            .app_data(pool.clone())
            .app_data(project_repo.clone())
            .app_data(task_repo.clone())
            .app_data(user_repo.clone())
            .route("/", web::get().to(index))
            .service(
                web::scope("/api")
                    .service(
                        web::resource("/projects")
                            .route(web::get().to(get_projects))
                            .route(web::post().to(create_project)),
                    )
            )
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}