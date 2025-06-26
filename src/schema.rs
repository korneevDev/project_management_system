use diesel::table;
use diesel::joinable;

table! {
    users (id) {
        id -> Integer,
        email -> Varchar,
        password_hash -> Varchar,
        role -> Varchar,
        created_at -> Timestamp,
    }
}

table! {
    projects (id) {
        id -> Integer,
        name -> Varchar,
        description -> Nullable<Text>,
        deadline -> Nullable<Timestamp>,
        status -> Varchar,
        created_by -> Integer,
        created_at -> Timestamp,
    }
}

table! {
    tasks (id) {
        id -> Integer,
        project_id -> Integer,
        title -> Varchar,
        description -> Nullable<Text>,
        status -> Varchar,
        priority -> Nullable<Varchar>,
        assignee_id -> Nullable<Integer>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    task_history (id) {
        id -> Integer,
        task_id -> Integer,
        changed_by -> Integer,
        field_name -> Varchar,
        old_value -> Nullable<Text>,
        new_value -> Nullable<Text>,
        changed_at -> Timestamp,
    }
}

table! {
    user_sessions (id) {
        id -> Integer,
        user_id -> Integer,
        token -> Varchar,
        expires_at -> Timestamp,
        created_at -> Timestamp,
    }
}

joinable!(tasks -> projects (project_id));
joinable!(tasks -> users (assignee_id));
joinable!(task_history -> tasks (task_id));
joinable!(task_history -> users (changed_by));
joinable!(user_sessions -> users (user_id));