use crate::{models::Project, schema::projects};
use diesel::prelude::*;
use super::DbPool;

pub struct ProjectRepository {
    pool: DbPool,
}

impl ProjectRepository {
    pub fn new(pool: DbPool) -> Self {
        Self { pool }
    }

    pub fn create_project(&self, name: &str, description: Option<&str>, creator_id: i32) -> Result<Project, diesel::result::Error> {
        let mut conn = self.pool.get().unwrap();
        diesel::insert_into(projects::table)
            .values((
                projects::name.eq(name),
                projects::description.eq(description),
                projects::created_by.eq(creator_id),
            ))
            .get_result(&mut conn)
    }

    pub fn get_project_by_id(&self, project_id: i32) -> Option<Project> {
        use crate::schema::projects::dsl::*;

        let mut conn = self.pool.get().unwrap();
        projects
            .filter(id.eq(project_id))
            .first(&mut conn)
            .optional()
            .unwrap()
    }

    pub fn list_projects(&self, status_filter: Option<&str>) -> Vec<Project> {
        use crate::schema::projects::dsl::*;

        let mut conn = self.pool.get().unwrap();
        let mut query = projects.into_boxed();

        if let Some(status) = status_filter {
            query = query.filter(status.eq(status));
        }

        query.load(&mut conn).unwrap()
    }
}