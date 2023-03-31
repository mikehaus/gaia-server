mod server;

// External Deps
use actix_web::{web, App, HttpServer};
use std::io::Result;

// Internal Deps
use server::{echo, hello, manual_hello, open_ai_completion};

// TODO: use this: https://docs.rs/actix-cors/latest/actix_cors/
#[actix_web::main]
async fn main() -> Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(hello)
            .service(echo)
            .service(open_ai_completion)
            .route("/hey", web::get().to(manual_hello))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
