// External Deps
use actix_web::{get, post, HttpResponse, Responder};
use std::io;

// Internal Deps

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
#[post("/open-ai/completions")]
pub async fn open_ai_completion() -> impl Responder {
    HttpResponse::Ok().body("This is a stubbed response")
}

// MARK: Manual Endpoints

pub async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there")
}
