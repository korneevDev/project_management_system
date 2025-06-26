use diesel::PgConnection;
use diesel::r2d2::{ConnectionManager, Pool};

pub type DbPool = Pool<ConnectionManager<PgConnection>>;

pub struct Repository {
    pub users: UserRepository,
    pub projects: ProjectRepository,
    pub tasks: TaskRepository,
}

impl Repository {
    pub fn new(pool: DbPool) -> Self {
        Self {
            users: UserRepository::new(pool.clone()),
            projects: ProjectRepository::new(pool.clone()),
            tasks: TaskRepository::new(pool),
        }
    }
}