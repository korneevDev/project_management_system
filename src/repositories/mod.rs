use diesel::r2d2::{ConnectionManager, Pool};
use diesel::PgConnection;

pub type DbPool = Pool<ConnectionManager<PgConnection>>;

pub mod user_repo;
pub mod project_repo;
pub mod task_repo;
pub mod auth_repo;

pub use user_repo::UserRepository;
pub use project_repo::ProjectRepository;
pub use task_repo::TaskRepository;
pub use auth_repo::AuthRepository;

#[derive(Clone)]
pub struct Repository {
    pub users: UserRepository,
    pub projects: ProjectRepository,
    pub tasks: TaskRepository,
    pub auth: AuthRepository,
}

impl Repository {
    pub fn new(pool: DbPool, jwt_secret: String, jwt_expiry_hours: i64) -> Self {
        Self {
            users: UserRepository::new(pool.clone()),
            projects: ProjectRepository::new(pool.clone()),
            tasks: TaskRepository::new(pool.clone()),
            auth: AuthRepository::new(pool, jwt_secret, jwt_expiry_hours),
        }
    }
}