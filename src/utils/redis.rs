use crate::models::redis::RedisClient;
use anyhow::Result;
use redis::{aio::MultiplexedConnection, AsyncCommands};

impl RedisClient {
    pub fn new() -> Result<Self> {
        let client = redis::Client::open("redis://127.0.0.1/")?;
        Ok(Self { client })
    }

    async fn get_connection(&self) -> Result<MultiplexedConnection> {
        let con = self.client.get_multiplexed_async_connection().await?;
        Ok(con)
    }

    pub async fn set_value(&self, key: &str, value: &str, expiration_secs: u64) -> Result<()> {
        let mut con = self.get_connection().await?;
        let _: () = con.set_ex(key, value, expiration_secs).await?;
        Ok(())
    }

    pub async fn remove_value(&self, key: &str) -> Result<()> {
        let mut con = self.get_connection().await?;
        let _: () = con.del(key).await?;
        Ok(())
    }

    pub async fn get_value(&self, key: &str) -> Result<Option<String>> {
        let mut con = self.get_connection().await?;
        let value: Option<String> = con.get(key).await?;
        Ok(value)
    }

    pub async fn get_ttl(&self, key: &str) -> Result<Option<i64>> {
        let mut con = self.get_connection().await?;

        let ttl: i64 = con.ttl(key).await?;

        if ttl == -2 {
            Ok(None)
        } else if ttl == -1 {
            Ok(None)
        } else {
            Ok(Some(ttl))
        }
    }
}
