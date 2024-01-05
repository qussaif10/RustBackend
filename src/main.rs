use actix_web::{web, App, HttpResponse, HttpServer, Responder, http};
use actix_cors::Cors;
use serde::{Serialize, Deserialize};

// Define a struct to deserialize the JSON request body
#[derive(Deserialize)]
struct LoginRequest {
    username: String,
    password: String,
}

// Define a struct for the response
#[derive(Serialize)]
struct LoginResponse {
    valid_credentials: bool,
}

// Define a handler for the login route
async fn login(info: web::Json<LoginRequest>) -> impl Responder {
    // Predefined credentials for demonstration purposes
    let expected_username = "admin";
    let expected_password = "password123";

    // Check credentials
    let are_credentials_valid = info.username == expected_username && info.password == expected_password;

    // Respond with a JSON object containing the `valid_credentials` field
    HttpResponse::Ok().json(LoginResponse { valid_credentials: are_credentials_valid })
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        // Configure CORS
        let cors = Cors::default()
            .allowed_origin("http://localhost:5173") // Adjust this to match your React app's URL
            .allowed_methods(vec!["GET", "POST"])
            .allowed_headers(vec![http::header::AUTHORIZATION, http::header::ACCEPT])
            .allowed_header(http::header::CONTENT_TYPE)
            .max_age(3600);

        App::new()
            .wrap(cors)
            .service(web::resource("/login").route(web::post().to(login)))
    })
        .bind("127.0.0.1:8080")?
        .run()
        .await
}
