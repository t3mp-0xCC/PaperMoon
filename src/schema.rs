// @generated automatically by Diesel CLI.

diesel::table! {
    posts (id) {
        id -> Uuid,
        content_id -> Varchar,
        title -> Varchar,
        content_html -> Varchar,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}
