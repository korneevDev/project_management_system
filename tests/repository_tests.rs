use project_management_system::{
    repositories::{Repository, DbPool},
    models::User,
    establish_test_connection
};
use diesel::r2d2::{ConnectionManager, Pool};
use diesel::RunQueryDsl; // Добавляем этот импорт
use bcrypt::{hash, DEFAULT_COST};

fn setup_test_db() -> DbPool {
    let manager = ConnectionManager::<diesel::PgConnection>::new(
        "postgres://postgres:root@localhost:5433/project_management_test"
    );
    let pool = Pool::builder()
        .max_size(1)
        .build(manager)
        .expect("Failed to create pool");
    
    // Очищаем и подготавливаем БД
    let mut conn = pool.get().unwrap();
    diesel::sql_query("TRUNCATE TABLE users CASCADE")
        .execute(&mut conn)
        .unwrap();
    pool
}


#[test]
fn test_user_creation_and_find() {
    let pool = setup_test_db();
    let repo = Repository::new(pool, "test_secret".to_string(), 24);
    
    // Используем роль, которая точно разрешена (например, "admin" или "member")
    let test_email = "test@example.com";
    let password = "test_password";
    let hashed_password = hash(password, DEFAULT_COST).unwrap();
    let allowed_role = "admin"; // Используем роль из допустимых значений
    
    // Создаем пользователя
    let user = repo.users.create_user(test_email, &hashed_password, allowed_role)
        .expect("Failed to create user");
    
    // Ищем пользователя
    let found_user = repo.users.find_by_email(test_email)
        .expect("Failed to find user");
    
    assert_eq!(user.id, found_user.id);
    assert_eq!(user.email, found_user.email);
    assert_eq!(user.role, found_user.role);
}

#[test]
fn test_find_nonexistent_user() {
    let pool = setup_test_db();
    let repo = Repository::new(pool, "test_secret".to_string(), 24);
    
    // Пытаемся найти несуществующего пользователя
    let result = repo.users.find_by_email("nonexistent@example.com");
    
    // Должны получить None
    assert!(result.is_none());
}

#[test]
fn test_create_session_success() {
    // Проверяем успешное создание сессии
}

#[test]
fn test_find_existing_session() {
    // Поиск существующей активной сессии
}

#[test]
fn test_create_task_success() {
    // Создание задачи с валидными данными
}

#[test]
fn test_create_task_with_nonexistent_project() {
    // Попытка создать задачу для несуществующего проекта
}

#[test]
fn test_create_task_with_invalid_assignee() {
    // Создание задачи с несуществующим исполнителем
}

#[test]
fn test_update_task_status_success() {
    // Успешное обновление статуса
}

#[test]
fn test_update_status_nonexistent_task() {
    // Попытка обновить несуществующую задачу
}

#[test]
fn test_update_status_with_history() {
    // Проверка записи в историю изменений
}

#[test]
fn test_get_task_history_multiple_changes() {
    // Получение истории после нескольких изменений
}

#[test]
fn test_get_history_for_nonexistent_task() {
    // Запрос истории для несуществующей задачи
}