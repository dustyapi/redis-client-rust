use redis::{aio::MultiplexedConnection, AsyncCommands, Client, RedisResult};
use std::sync::Arc;
use tokio::sync::Mutex;

pub struct RedisClient {
    client: Arc<Mutex<Client>>,
}

impl RedisClient {
    pub fn new(redis_url: &str) -> Self {
        let client = Client::open(redis_url).expect("Failed to create Redis client");
        Self {
            client: Arc::new(Mutex::new(client)),
        }
    }

    async fn get_connection(&self) -> MultiplexedConnection {
        let client = self.client.lock().await;
        client.get_multiplexed_async_connection().await.expect("Failed to connect to Redis")
    }

    pub async fn set(&self, key: &str, value: &str) -> RedisResult<()> {
        let mut connection = self.get_connection().await;
        connection.set(key, value).await
    }

    pub async fn get(&self, key: &str) -> RedisResult<String> {
        let mut connection = self.get_connection().await;
        connection.get(key).await
    }
}
