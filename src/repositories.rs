use crate::models::{Project, NewProject, Task, NewTask, User, NewUser};
use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, Pool};

pub struct ProjectRepository {
    pool: Pool<ConnectionManager<PgConnection>>,
}

impl ProjectRepository {
    pub fn new(pool: Pool<ConnectionManager<PgConnection>>) -> Self {
        Self { pool }
    }

    pub fn create(&self, new_project: NewProject) -> Result<Project, diesel::result::Error> {
        use crate::schema::projects;
        
        let mut conn = self.pool.get().unwrap();
        diesel::insert_into(projects::table)
            .values(&new_project)
            .get_result(&mut conn)
    }

    pub fn find(&self, id: i32) -> Result<Project, diesel::result::Error> {
        use crate::schema::projects::dsl::*;
        
        let mut conn = self.pool.get().unwrap();
        projects.find(id).first(&mut conn)
    }

    pub fn all(&self) -> Result<Vec<Project>, diesel::result::Error> {
        use crate::schema::projects::dsl::*;
        
        let mut conn = self.pool.get().unwrap();
        projects.load::<Project>(&mut conn)
    }

    pub fn update(&self, id: i32, project: NewProject) -> Result<Project, diesel::result::Error> {
        use crate::schema::projects::dsl::*;
        
        let mut conn = self.pool.get().unwrap();
        diesel::update(projects.find(id))
            .set(&project)
            .get_result(&mut conn)
    }

    pub fn delete(&self, id: i32) -> Result<usize, diesel::result::Error> {
        use crate::schema::projects::dsl::*;
        
        let mut conn = self.pool.get().unwrap();
        diesel::delete(projects.find(id)).execute(&mut conn)
    }
}
