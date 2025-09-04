#[macro_use]
extern crate validator_derive;

use actix_cors::Cors;
use actix_web::http::header;
use actix_web::{middleware, web, App, HttpServer};
use color_eyre::Result;
use config::Config;
use tracing::info;
use routes::{hello, echo, manual_hello};

mod config;
mod handlers;
mod models;
mod db;
mod routes;

#[actix_web::main]
async fn main() -> Result<()> {
    let config = Config::from_env().expect("Failed to load environment variables!!!");

    let pool = config.db_pool().await?;
    let pool_state = web::Data::new(pool);

    let crypto_service = config.crypto_service();

    info!("Starting server at http://{}:{}", config.host, config.port);
    
    HttpServer::new(move || {
        let cors = Cors::default()
            .allow_any_origin()
            .allowed_methods(vec!["GET", "POST", "PATCH", "PUT", "DELETE"])
            .allowed_headers(vec![
                               header::CONTENT_TYPE,
                               header::AUTHORIZATION,
                               header::ACCEPT,
            ])
            .supports_credentials();
        
        App::new()
            .wrap(middleware::Logger::default())
            .wrap(cors)
            .app_data(pool_state.clone())
            .app_data(crypto_service.clone())
            .service(hello)
            .service(echo)
            .route("/hey", web::get().to(manual_hello))
            .configure(handlers::app_config)
    })
    .bind(format!("{}:{}", config.host, config.port))?
        .run()
        .await?;
    
    Ok(())
}
