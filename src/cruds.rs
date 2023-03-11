use anyhow::Context;
use diesel::{insert_into, RunQueryDsl};

use crate::db::establish_connection;
use crate::models::NewPost;
use crate::schema::posts;

pub fn insert_new_post (
    title: &String
) -> anyhow::Result<()> {
    let conn = &mut establish_connection()?;
    let new_post = NewPost{title};
    insert_into(posts::dsl::posts).values(&new_post)
        .execute(conn)
        .with_context(|| "Failed to insert new_post")?;
    Ok(())
}
