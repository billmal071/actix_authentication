pub mod crypto;

use std::sync::Arc;

use color_eyre::Result;
use config::ConfigError;
use dotenv::dotenv;
use eyre::WrapErr;
use serde::Deserialize;
use sqlx::PgPool;
use tracing::{info, instrument};
use tracing_subscriber::filter::EnvFilter;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub host: String,
    pub port: i32,
    pub database_url: String,
    pub secret_key: String,
}

impl Config {
    #[instrument]
    pub fn from_env() -> Result<Config, ConfigError> {
        dotenv().ok();

        tracing_subscriber::fmt()
            .with_env_filter(EnvFilter::from_default_env())
            .init();

        info!("Loading configuration!");

        let settings = config::Config::builder()
            .add_source(config::Environment::default())
            .build()?;

        settings.try_deserialize::<Config>() 
    }

    pub async fn db_pool(&self) -> Result<PgPool> {
       info!("Creating database pool!"); 

        PgPool::connect(&self.database_url)
            .await
            .context("Creating database pool!")
    }

    pub fn crypto_service(&self) -> crypto::CryptoService {
        crypto::CryptoService {
           key: Arc::new(self.secret_key.clone()), 
        }
    }
}
