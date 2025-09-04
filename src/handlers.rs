use actix_web::web::{self, ServiceConfig};
use actix_web::HttpResponse;

pub fn app_config(config: &mut ServiceConfig) {
    let health_resource = web::resource("/health")
        .route(web::get().to(health));
    
    config.service(health_resource);
}

pub async fn health() -> HttpResponse {
    HttpResponse::Ok().finish()
}
