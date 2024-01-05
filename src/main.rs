use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use serde::Deserialize;

// Define a struct to deserialize the JSON request body
#[derive(Deserialize)]
struct LoginRequest {
    username: String,
    password: String,
}

// Define a handler for the login route
async fn login(info: web::Json<LoginRequest>) -> impl Responder {
    // You can access the username and password here
    // For example: info.username and info.password

    // Respond with a simple message
    HttpResponse::Ok().body(format!("Received login for user: {}", info.username))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(web::resource("/login").route(web::post().to(login)))
    })
        .bind("127.0.0.1:8080")?
        .run()
        .await
}
