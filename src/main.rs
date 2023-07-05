use actix_cors::Cors;
use actix_files::Files;
use actix_web::{
    http::StatusCode,
    App,
    HttpServer,
};
use actix_web::middleware::{
    Logger,
    ErrorHandlers,
};
use dotenv::dotenv;
use env_logger::Env;
use std::path::Path;
use std::fs;

extern crate log;

mod article;
mod cruds;
mod db;
mod error;
mod models;
mod router;
mod schema;
mod watcher;

use error::render_error_page as render_error_page;

macro_rules! error_handler_many {
    ($handler:ident, [$($variant:ident),*]) => {
        ErrorHandlers::new()
            $(.handler(StatusCode::$variant, $handler))+
    }
}


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    /* Init */
    env_logger::init_from_env(Env::default().default_filter_or("debug"));
    dotenv().ok();
    /* Watcher */
    let md_folder = Path::new("article_md/");
    if md_folder.exists() == false {
        fs::create_dir_all("article_md")?;
    }
    tokio::spawn(watcher::async_watch(md_folder));
    /* Start HTTP Server */
    HttpServer::new(|| {
        let cors = Cors::permissive()
            .max_age(3600);
        App::new()
            .wrap(cors)
            .wrap(Logger::default())
            .wrap(error_handler_many!(render_error_page, [BAD_REQUEST, UNAUTHORIZED, FORBIDDEN,
            NOT_FOUND, METHOD_NOT_ALLOWED, NOT_ACCEPTABLE, REQUEST_TIMEOUT, GONE,
            LENGTH_REQUIRED, PAYLOAD_TOO_LARGE, URI_TOO_LONG, UNSUPPORTED_MEDIA_TYPE,
            RANGE_NOT_SATISFIABLE, IM_A_TEAPOT, TOO_MANY_REQUESTS,
            REQUEST_HEADER_FIELDS_TOO_LARGE, MISDIRECTED_REQUEST, UPGRADE_REQUIRED,
            INTERNAL_SERVER_ERROR, NOT_IMPLEMENTED, SERVICE_UNAVAILABLE,
            HTTP_VERSION_NOT_SUPPORTED]))
            .service(router::index)
            .service(router::article)
            .service(router::article_list)
            .service(router::about)
            .service(router::portfolio)
            .service(router::joke)
            .service(Files::new("/misc", "./misc").show_files_listing())
            .service(Files::new("/static", "./static").show_files_listing())
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
