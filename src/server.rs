// External Deps
use actix_web::{get, http::header::ContentType, post, HttpResponse, Responder};

// Internal Deps
#[path = "./services/mod.rs"]
pub mod services;

#[path = "./client.rs"]
mod client;

// MARK: GET

#[get("/")]
pub async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

// MARK: POST

#[post("/echo")]
pub async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

// TODO: Implement completion endpoint once completions logic implemented
#[post("/openai/completions")]
pub async fn open_ai_completion(req_body: String) -> impl Responder {
    let client = client::client_builder();

    // dbg!(&req_body);

    let response = services::completion_handler::generate_completion(client, req_body).await;
    match response {
        Ok(text) => HttpResponse::Ok()
            .content_type(ContentType::plaintext())
            .insert_header(("X-Hdr", "sample"))
            .body(text),
        Err(err) => {
            println!("{}", err);
            HttpResponse::Ok()
                .body("There was an error while generating a text completion from this endpoint")
        }
    }
}

#[post("/openai/images")]
pub async fn open_ai_image_generation() -> impl Responder {
    let client = client::client_builder();

    let response = services::images_handler::generate_images(client).await;
    match response {
        Ok(text) => HttpResponse::Ok()
            .content_type(ContentType::plaintext())
            .insert_header(("X-Hdr", "sample"))
            .body(text),
        Err(err) => {
            println!("{}", err);
            HttpResponse::Ok()
                .body("An error occurred while generating an image from this endpoint")
        }
    }
}

// MARK: Manual Endpoints

pub async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there")
}
