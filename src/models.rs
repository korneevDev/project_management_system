use chrono::NaiveDateTime;
use serde::{Serialize, Deserialize};
use diesel::prelude::*;

#[derive(Queryable, Serialize)]
pub struct User {
    pub id: i32,
    pub email: String,
    pub password_hash: String,
    pub role: String,
    pub created_at: chrono::NaiveDateTime,
}

#[derive(Insertable)]
#[diesel(table_name = crate::schema::users)]
pub struct NewUser {
    pub email: String,
    pub password_hash: String,
    pub role: String,
}

#[derive(Queryable, Serialize)]
pub struct Project {
    pub id: i32,
    pub name: String,
    pub description: Option<String>,
    pub deadline: Option<NaiveDateTime>,
    pub status: String,
    pub created_by: i32,
    pub created_at: NaiveDateTime,
}

#[derive(Insertable, Deserialize)]
#[diesel(table_name = crate::schema::projects)]
pub struct NewProject {
    pub name: String,
    pub description: Option<String>,
    pub created_by: i32,
}

#[derive(Queryable, Serialize)]
pub struct Task {
    pub id: i32,
    pub project_id: i32,
    pub title: String,
    pub description: Option<String>,
    pub status: String,
    pub priority: Option<String>,
    pub assignee_id: Option<i32>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Insertable, Deserialize)]
#[diesel(table_name = crate::schema::tasks)]
pub struct NewTask {
    pub title: String,
    pub project_id: i32,
    pub assignee_id: Option<i32>,
}

#[derive(Queryable, Serialize)]
pub struct TaskHistory {
    pub id: i32,
    pub task_id: i32,
    pub changed_by: i32,
    pub field_name: String,
    pub old_value: Option<String>,
    pub new_value: Option<String>,
    pub changed_at: NaiveDateTime,
}

#[derive(Queryable, Identifiable, Serialize)]
#[diesel(table_name = crate::schema::user_sessions)]
pub struct UserSession {
    pub id: i32,
    pub user_id: i32,
    pub token: String,
    pub expires_at: NaiveDateTime,
    pub created_at: NaiveDateTime,
}

#[derive(Insertable)]
#[diesel(table_name = crate::schema::user_sessions)]
pub struct NewUserSession {
    pub user_id: i32,
    pub token: String,
    pub expires_at: NaiveDateTime,
}