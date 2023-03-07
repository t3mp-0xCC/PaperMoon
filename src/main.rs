use std::env;

use actix_web::{
    HttpServer,
    App
};
use dotenv::dotenv;

mod router;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    /* Init */
    std::env::set_var("RUST_LOG", "actix_web=info");
    dotenv().ok();
    /* Start HTTP Server */
    HttpServer::new(|| {
        App::new()
            .service(router::index)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
