use actix_web::{
    get,
    Error,
    HttpResponse,
    HttpRequest,
};
use tera::Tera;
use log::debug;

use crate::{cruds, error};
use crate::error::error_page_response;

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
        Err(_) => return Ok(error_page_response(500, "Something went wrong in Tera Template")),
    };
    let mut tera_ctx = tera::Context::new();
    let posts = match cruds::get_post_list(){
        Ok(v) => v,
        Err(_) => return Ok(error_page_response(500, "Failed to get posts list from DB")),
    };
    tera_ctx.insert("posts", &posts);
    let view = match tera.render("article_list.html.tera", &tera_ctx) {
        Ok(v) => v,
        Err(_) => return Ok(error_page_response(500, "Something went wrong in Tera Rendering")),
    };

    Ok(HttpResponse::Ok().content_type("text/html").body(view))
}

#[get("/article/{article_id}")]
async fn article(req: HttpRequest) -> Result<HttpResponse, Error> {
    let matcher = req.match_info();
    let article_id = match matcher.get("article_id") {
        Some(id) => id,
        None => return Ok(error_page_response(404, "article_id is missing")),
    };
    debug!("article_id: {:?}", article_id);
    let article = match cruds::get_post_from_content_id(&article_id.to_string()) {
        Ok(a) => a,
        Err(_) => return Ok(error_page_response(404, &format!("{} is invalid article_id", article_id))),
    };
    debug!("article found: {}", article.title);

    /* Tera */
    let tera = match Tera::new("templates/*") {
        Ok(t) => t,
        Err(_) => return Ok(error_page_response(500, "Something went wrong in Tera Template")),
    };

    let mut tera_ctx = tera::Context::new();
    // key, value
    tera_ctx.insert("title", &article.title);
    tera_ctx.insert("article", &article.content_html);
    let view = match tera.render("article.html.tera", &tera_ctx) {
        Ok(v) => v,
        Err(_) => return Ok(error_page_response(500, "Something wrong in Tera rendering")),

    };

    Ok(HttpResponse::Ok().content_type("text/html").body(view))
}

/* about */
#[get("/about")]
async fn about() -> Result<HttpResponse, Error> {
    Ok(HttpResponse::Ok().content_type("text/html").body("about"))
}

/* portfolio */
#[get("/portfolio")]
async fn portfolio() -> Result<HttpResponse, Error> {
    Ok(HttpResponse::Ok().content_type("text/html").body("portfolio"))
}

