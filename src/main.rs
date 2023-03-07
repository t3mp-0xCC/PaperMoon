use actix_web::{
    HttpServer,
    App
};
use actix_web::middleware::Logger;
use dotenv::dotenv;
use env_logger::Env;

mod router;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    /* Init */
    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init_from_env(Env::default().default_filter_or("debug"));
    dotenv().ok();
    /* Start HTTP Server */
    HttpServer::new(|| {
        App::new()
            .wrap(Logger::default())
            .service(router::index)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
