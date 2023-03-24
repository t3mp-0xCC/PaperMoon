use actix_cors::Cors;
use actix_files::Files;
use actix_web::{
    App,
    HttpServer,
};
use actix_web::middleware::Logger;
use dotenv::dotenv;
use env_logger::Env;
use std::path::Path;
use std::fs;

extern crate log;

mod article;
mod cruds;
mod db;
mod models;
mod router;
mod schema;
mod watcher;

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
            .service(router::index)
            .service(router::article)
            .service(router::article_list)
            .service(Files::new("/misc", "./misc").show_files_listing())
            .service(Files::new("/static", "./static").show_files_listing())
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
