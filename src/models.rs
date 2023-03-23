use chrono::NaiveDateTime;
use diesel::prelude::{Queryable, Identifiable, Selectable, Insertable};
use serde::Serialize;
use uuid::Uuid;

use crate::schema::posts;

#[derive(Queryable, Serialize, Identifiable, Selectable, PartialEq, Debug, Clone)]
#[diesel(table_name = posts)]
pub struct Post {
    pub id: Uuid,
    pub content_id: String,
    pub title: String,
    pub content_html: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Insertable)]
#[diesel(table_name = posts)]
pub struct  NewPost<'a> {
    pub content_id: &'a String,
    pub title: &'a String,
    pub content_html: &'a String,
}

