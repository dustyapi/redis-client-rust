
use serde::Deserialize;
use dotenv::dotenv;
use std::env;

#[derive(Deserialize)]
pub struct AppConfig {
    pub redis_url: String,
}

impl AppConfig {
  pub fn from_env() -> Self {
        dotenv().ok();

        let redis_url = env::var("REDIS_URL").expect("REDIS_URL must be set");

        AppConfig { redis_url }
    }
}
