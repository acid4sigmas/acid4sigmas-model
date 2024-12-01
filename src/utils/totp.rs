use anyhow::{anyhow, Result};
use std::time::{SystemTime, UNIX_EPOCH};
use totp_rs::{Algorithm, TOTP};

use crate::{
    models::{
        auth::TwoFaType,
        redis::RedisClient,
        totp::{TotpGen, TotpStorage},
    },
    to_string_,
};

impl TotpGen {
    pub fn new(secret: &str) -> Self {
        Self {
            secret: to_string_!(secret),
        }
    }

    pub fn generate_totp(&self) -> Result<String> {
        let totp = TOTP::new(Algorithm::SHA1, 6, 1, 30, self.secret.as_bytes().to_vec())
            .map_err(|_| anyhow!("failed to create TOTP instance"))?;

        let time_now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map_err(|_| anyhow!("System time error"))?
            .as_secs();

        let totp_code = totp.generate(time_now);

        Ok(totp_code)
    }
}

impl TotpStorage {
    pub fn new(user_id: i64, totp: Option<String>, twofa_type: TwoFaType) -> Result<Self> {
        let con = RedisClient::new()?;
        let redis_key = format!("{}:{}", twofa_type.as_str(), user_id);

        Ok(Self {
            totp,
            redis_client: con,
            redis_key,
        })
    }

    pub async fn store(&self) -> Result<()> {
        let redis_client = &self.redis_client;

        if let Some(ttl) = redis_client.get_ttl(&self.redis_key).await? {
            let cool_down_time = 60;
            let min_ttl = 600 - cool_down_time;

            if ttl >= min_ttl {
                let remaining_wait_time = ttl - min_ttl;
                return Err(anyhow!(
                    "please request a new verification code in {}",
                    remaining_wait_time
                ));
            }
        }

        if let Some(totp) = &self.totp {
            redis_client
                .set_value(&self.redis_key, totp, 600)
                .await
                .map_err(|e| anyhow!("failed to store totp: {}", e.to_string()))?;
        } else {
            return Err(anyhow!(
                "please provide a totp in TotpStorage::new(..., Some(totp), ...)"
            ));
        }

        Ok(())
    }

    pub async fn get(&self) -> Result<Option<String>> {
        let redis_client = &self.redis_client;

        let value = redis_client.get_value(&self.redis_key).await?;

        Ok(value)
    }

    pub async fn remove(&self) -> Result<()> {
        let redis_client = &self.redis_client;

        let _ = redis_client.remove_value(&self.redis_key).await;

        Ok(())
    }
}
