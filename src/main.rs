mod config;
mod redis;

use tokio;
use config::AppConfig;
use crate::redis::RedisClient;

#[tokio::main]
async fn main() {
    let config = AppConfig::from_env();

    let redis_client: RedisClient = RedisClient::new(&config.redis_url);
    let key = "test";
    let value = "test123";
    match redis_client.set(key, value).await {
        Ok(value) => println!("Inserted {:?}", value),
        Err(err) => println!("Failed to get insert: {:?}", err),
    }
    match redis_client.get(key).await {
        Ok(value) => println!("The value for {:?} is: {:?}", key, value),
        Err(err) => println!("Failed to get value from Redis: {:?}", err),
    }
}
