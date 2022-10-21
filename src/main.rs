mod mailer;
mod mail;
mod routes;

use dotenv::dotenv;
use std::env;
use actix_web::{App, HttpServer};
use routes::{root, send};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let port = env::var("PORT").expect("PORT not set");

    HttpServer::new(move ||{
        App::new()
            .service(root)
            .service(send)
    })
        .bind(format!("0.0.0.0:{}", &port))
        .unwrap()
        .run()
        .await
}
