mod validators;

use actix_cors::Cors;
use actix_web::{web, App, HttpServer, Responder, HttpResponse};
use serde::{Deserialize, Serialize};
use validators::AddressValidator;

#[derive(Deserialize)]
struct AddressInput {
    address: String,
    blockchain: Option<String>,
}

#[derive(Serialize)]
struct ValidationResponse {
    status: String,
    blockchain: Option<String>,
    error: Option<String>,
}

async fn validate_address(input: web::Json<AddressInput>) -> impl Responder {
    let address = &input.address;
    let blockchain = input.blockchain.as_deref();

    match AddressValidator::validate(address, blockchain) {
        Ok(blockchain_type) => HttpResponse::Ok().json(ValidationResponse {
            status: "valid".to_string(),
            blockchain: Some(format!("{:?}", blockchain_type)),
            error: None,
        }),
        Err(err) => HttpResponse::BadRequest().json(ValidationResponse {
            status: "invalid".to_string(),
            blockchain: None,
            error: Some(err.to_string()),
        }),
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Server starting on http://localhost:8080");
    HttpServer::new(|| {
        let cors = Cors::default()
            .allow_any_origin()
            .allow_any_method()
            .allow_any_header();

        App::new()
            .wrap(cors)
            .service(web::resource("/validate").route(web::post().to(validate_address)))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}