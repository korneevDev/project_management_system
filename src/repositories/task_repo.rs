use crate::{models::{Task, TaskHistory}, schema::{tasks, task_history}};
use diesel::prelude::*;
use super::DbPool;

pub struct TaskRepository {
    pool: DbPool,
}

impl TaskRepository {
    pub fn new(pool: DbPool) -> Self {
        Self { pool }
    }

    pub fn create_task(
        &self,
        title: &str,
        project_id: i32,
        creator_id: i32,
    ) -> Result<Task, diesel::result::Error> {
        let mut conn = self.pool.get().unwrap();
        diesel::insert_into(tasks::table)
            .values((
                tasks::title.eq(title),
                tasks::project_id.eq(project_id),
                tasks::assignee_id.eq(creator_id), // По умолчанию создатель = исполнитель
            ))
            .get_result(&mut conn)
    }

    pub fn update_task_status(
        &self,
        task_id: i32,
        new_status: &str,
        updater_id: i32,
    ) -> Result<(Task, TaskHistory), diesel::result::Error> {
        let mut conn = self.pool.get().unwrap();
        conn.transaction(|conn| {
            // Обновляем статус задачи
            let updated_task = diesel::update(tasks::table.find(task_id))
                .set(tasks::status.eq(new_status))
                .get_result::<Task>(conn)?;

            // Записываем в историю
            let history_entry = diesel::insert_into(task_history::table)
                .values((
                    task_history::task_id.eq(task_id),
                    task_history::changed_by.eq(updater_id),
                    task_history::field_name.eq("status"),
                    task_history::new_value.eq(new_status),
                ))
                .get_result::<TaskHistory>(conn)?;

            Ok((updated_task, history_entry))
        })
    }

    pub fn get_task_history(&self, task_id: i32) -> Vec<TaskHistory> {
        task_history::table
            .filter(task_history::task_id.eq(task_id))
            .order(task_history::changed_at.desc())
            .load(&mut self.pool.get().unwrap())
            .unwrap()
    }
}