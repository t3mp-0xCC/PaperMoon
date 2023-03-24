use actix_web::{
    get,
    Error,
    HttpResponse,
    HttpRequest,
};
use tera::Tera;
use log::debug;

use crate::cruds;

/* index */
#[get("/")]
async fn index() -> Result<HttpResponse, Error> {
    Ok(HttpResponse::Ok().content_type("text/html").body("index"))
}


/* article */
#[get("/article")]
async fn article_list() -> Result<HttpResponse, Error> {
    /* Tera */
    let tera = match Tera::new("templates/*") {
        Ok(t) => t,
        Err(_) => return Ok(HttpResponse::InternalServerError().body("Something wrong in Tera template")),
    };
    let mut tera_ctx = tera::Context::new();
    let posts = match cruds::get_post_list(){
        Ok(v) => v,
        Err(_) => return Ok(HttpResponse::InternalServerError().body("Failed to get postslist")),
    };
    tera_ctx.insert("posts", &posts);
    let view = match tera.render("article_list.html.tera", &tera_ctx) {
        Ok(v) => v,
        Err(_) => return Ok(HttpResponse::InternalServerError().body("Something wrong in Tera rendering")),
    };

    Ok(HttpResponse::Ok().content_type("text/html").body(view))
}

#[get("/article/{article_id}")]
async fn article(req: HttpRequest) -> Result<HttpResponse, Error> {
    let matcher = req.match_info();
    let article_id = match matcher.get("article_id") {
        Some(id) => id,
        None => return Ok(HttpResponse::NotFound().body("article_id is missing")),
    };
    debug!("article_id: {:?}", article_id);
    let article = match cruds::get_post_from_content_id(&article_id.to_string()) {
        Ok(a) => a,
        Err(_) => return Ok(HttpResponse::NotFound().body(format!("{} is invalid article_id", article_id))),
    };
    debug!("article found: {}", article.title);

    /* Tera */
    let tera = match Tera::new("templates/*") {
        Ok(t) => t,
        Err(_) => return Ok(HttpResponse::InternalServerError().body("Something wrong in Tera template")),
    };

    let mut tera_ctx = tera::Context::new();
    // key, value
    tera_ctx.insert("title", &article.title);
    tera_ctx.insert("article", &article.content_html);
    let view = match tera.render("article.html.tera", &tera_ctx) {
        Ok(v) => v,
        Err(_) => return Ok(HttpResponse::InternalServerError().body("Something wrong in Tera rendering")),

    };

    Ok(HttpResponse::Ok().content_type("text/html").body(view))
}
