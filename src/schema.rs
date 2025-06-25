table! {
    projects (id) {
        id -> Int4,
        name -> Varchar,
        description -> Text,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        status -> Varchar,
    }
}

table! {
    tasks (id) {
        id -> Int4,
        project_id -> Int4,
        title -> Varchar,
        description -> Text,
        status -> Varchar,
        priority -> Varchar,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        due_date -> Nullable<Timestamp>,
        assigned_to -> Nullable<Varchar>,
    }
}

table! {
    users (id) {
        id -> Int4,
        username -> Varchar,
        email -> Varchar,
        password_hash -> Varchar,
        created_at -> Timestamp,
        role -> Varchar,
    }
}

joinable!(tasks -> projects (project_id));