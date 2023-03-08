use actix_web::{
    get,
    Error,
    HttpResponse,
    HttpRequest,
};
use std::fs;
use std::path::Path;
use log::debug;

/* index */
#[get("/")]
async fn index() -> Result<HttpResponse, Error> {
    Ok(HttpResponse::Ok().content_type("text/html").body("index"))
}


/* article */
#[get("/article/{article_id}")]
async fn article(req: HttpRequest) -> Result<HttpResponse, Error> {
    let matcher = req.match_info();
    let article_id = match matcher.get("article_id") {
        Some(id) => id,
        None => return Ok(HttpResponse::NotFound().body("article_id is missing")),
    };
    debug!("article_id: {:?}", article_id);
    let path = Path::new("./article").join(format!("{}.html", article_id));
    debug!("article_path: {:?}", path);
    let content = match fs::read_to_string(path) {
        Ok(file) => file,
        Err(_) => return Ok(HttpResponse::NotFound().body(format!("{} is invalid article_id", article_id))),
    };
    Ok(HttpResponse::Ok().content_type("text/html").body(content))
}
