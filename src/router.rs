use actix_web::{
    HttpResponse,
    Error,
    get,
};

/* index */
#[get("/")]
async fn index() -> Result<HttpResponse, Error> {
    Ok(HttpResponse::Ok().content_type("text/html").body("index"))
}


/*
/* article */
#[get("/article/{article_id}")]
async fn article(path: web::Path<String>) -> Result<HttpResponse, Error> {
  let article_id = path.into_inner(); 
}
*/
