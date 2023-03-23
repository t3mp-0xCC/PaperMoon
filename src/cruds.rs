use anyhow::{anyhow, Context};
use diesel::{
    insert_into,
    ExpressionMethods,
    RunQueryDsl,
    QueryDsl
};

use crate::db::establish_connection;
use crate::models::{NewPost, Post};
use crate::schema::posts;

pub fn insert_new_post (
    content_id: &String,
    title: &String,
    content_html: &String
) -> anyhow::Result<()> {
    let conn = &mut establish_connection()?;
    let new_post = NewPost{content_id, title, content_html};
    insert_into(posts::dsl::posts).values(&new_post)
        .execute(conn)
        .with_context(|| "Failed to insert new_post")?;
    Ok(())
}

pub fn get_post_from_content_id(cid: &String) -> anyhow::Result<Post> {
    use crate::schema::posts::dsl::*;
    let conn = &mut establish_connection()?;
    match posts.filter(content_id.eq(cid)).first::<Post>(conn) {
        Ok(p) => Ok(p),
        Err(_) => return Err(anyhow!("Failed to filter posts")),
    }
}

pub fn update_post (
    content_id: &String,
    new_title: &String,
    new_content_html: &String,
) -> anyhow::Result<()> {
    let conn = &mut establish_connection()?;
    let target = posts::dsl::posts
        .filter(posts::dsl::content_id.eq(content_id));
    diesel::update(target)
        .set(posts::dsl::title.eq(new_title))
        .execute(conn)
        .with_context(|| "Failed to update title")?;
    diesel::update(target)
        .set(posts::dsl::content_html.eq(new_content_html))
        .execute(conn)
        .with_context(|| "Failed to update content_html")?;
    Ok(())
}

#[cfg(test)]
mod cruds_tests {
    use super::*;

    #[test]
    #[ignore]
    fn search_from_id() {
        let content_id = String::from("search_from_id");
        let title = String::from("Asylum Piece");
        let content_html = String::from("Knock Knock");
        insert_new_post(&content_id, &title, &content_html)
            .expect("Failed to insert test post");
        let test_post = get_post_from_id(&content_id)
            .expect("Failed to search from id");
        assert_eq!(test_post.title, title)
    }
}
