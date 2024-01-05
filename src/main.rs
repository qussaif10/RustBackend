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
    // Predefined credentials for demonstration purposes
    let expected_username = "admin";
    let expected_password = "password123";

    // Check credentials
    let are_credentials_valid = info.username == expected_username && info.password == expected_password;

    // Respond with a boolean value
    HttpResponse::Ok().json(are_credentials_valid)
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
