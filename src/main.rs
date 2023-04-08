mod server;

// External Deps
use actix_cors::Cors;
use actix_web::{http, web, App, HttpServer};
use std::io::Result;

// Internal Deps
use server::{echo, hello, manual_hello, open_ai_completion, open_ai_image_generation};

// TODO: use this: https://docs.rs/actix-cors/latest/actix_cors/
#[actix_web::main]
async fn main() -> Result<()> {
    HttpServer::new(|| {
        // let cors = Cors::default()
        //     .allowed_origin("*")
        //     .allowed_methods(vec!["GET", "POST"])
        //     .allowed_headers(vec![http::header::AUTHORIZATION, http::header::ACCEPT])
        //     .allowed_header(http::header::CONTENT_TYPE)
        //     .max_age(3600);

        // TODO: Figure out non-permissive cors policy
        // NOTE: on wrap(cors) app wasn't building
        App::new()
            .wrap(Cors::permissive())
            .service(hello)
            .service(echo)
            .service(open_ai_completion)
            .service(open_ai_image_generation)
            .route("/hey", web::get().to(manual_hello))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
