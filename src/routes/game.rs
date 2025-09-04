use actix_web::web::Json;
use actix_web::{post, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
use serde_json::json;

#[derive(Serialize, Deserialize)]
struct CreateGameSchema {
    name: String,
}

#[post("/game/")]
async fn create_game(body: Json<CreateGameSchema>) -> impl Responder {
   HttpResponse::Ok().json(json!({
             "status": "OK",
             "message": body
    })) 
}


