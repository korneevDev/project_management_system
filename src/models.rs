use diesel::prelude::*;
use chrono::NaiveDateTime;
use serde::{Serialize, Deserialize};

#[derive(Queryable, Serialize, Identifiable)]
pub struct Project {
    pub id: i32,
    pub name: String,
    pub description: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub status: String,
}

#[derive(Insertable, Deserialize)]
#[diesel(table_name = projects)]
pub struct NewProject {
    pub name: String,
    pub description: String,
    pub status: String,
}

#[derive(Queryable, Serialize, Identifiable, Associations)]
#[diesel(belongs_to(Project))]
pub struct Task {
    pub id: i32,
    pub project_id: i32,
    pub title: String,
    pub description: String,
    pub status: String,
    pub priority: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub due_date: Option<NaiveDateTime>,
    pub assigned_to: Option<String>,
}

#[derive(Insertable, Deserialize)]
#[diesel(table_name = tasks)]
pub struct NewTask {
    pub project_id: i32,
    pub title: String,
    pub description: String,
    pub status: String,
    pub priority: String,
    pub due_date: Option<NaiveDateTime>,
    pub assigned_to: Option<String>,
}

#[derive(Queryable, Serialize, Identifiable)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub email: String,
    pub password_hash: String,
    pub created_at: NaiveDateTime,
    pub role: String,
}

#[derive(Insertable, Deserialize)]
#[diesel(table_name = users)]
pub struct NewUser {
    pub username: String,
    pub email: String,
    pub password: String,
    pub role: String,
}