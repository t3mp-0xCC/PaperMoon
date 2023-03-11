use chrono::NaiveDateTime;
use diesel::prelude::{Queryable, Identifiable, Selectable, Insertable, QueryableByName};
use serde::{Serialize, Deserialize};
use uuid::Uuid;

use crate::schema::posts;

#[derive(Queryable, Serialize, Identifiable, Selectable, PartialEq, Debug, Clone)]
#[diesel(table_name = posts)]
pub struct Post {
    pub id: Uuid,
    pub title: String,
    pub content_html: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

