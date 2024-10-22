use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};

pub trait Claim {
    fn exp(&self) -> usize;
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct BackendClaims {
    pub exp: usize,
    pub timestamp: u64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UserClaims {
    pub user_id: String,
    pub exp: usize,
}

impl Claim for UserClaims {
    fn exp(&self) -> usize {
        self.exp
    }
}

impl Claim for BackendClaims {
    fn exp(&self) -> usize {
        self.exp
    }
}

pub struct JwtToken {
    secret: String,
}

type JwtError = jsonwebtoken::errors::Error;

impl JwtToken {
    pub fn new(secret_key: &str) -> Self {
        Self {
            secret: secret_key.to_string(),
        }
    }

    pub fn create_jwt<T: Claim + Serialize>(&self, claims: &T) -> Result<String, JwtError> {
        let header = Header::default();
        let encoding_key = EncodingKey::from_secret(self.secret.as_ref());
        encode(&header, claims, &encoding_key).into()
    }

    pub fn decode_jwt<T: for<'de> Deserialize<'de> + Claim>(
        &self,
        token: &str,
    ) -> Result<T, JwtError> {
        let decoding_key = DecodingKey::from_secret(self.secret.as_ref());
        let validation = Validation::default();
        let decoded = decode::<T>(token, &decoding_key, &validation)?;

        let claims = decoded.claims;

        if JwtToken::is_token_expired(claims.exp()) {
            return Err(jsonwebtoken::errors::Error::from(
                jsonwebtoken::errors::ErrorKind::ExpiredSignature,
            ));
        }

        Ok(claims)
    }

    fn is_token_expired(exp: usize) -> bool {
        let current_timestamp = JwtToken::get_current_timestamp();
        exp <= current_timestamp as usize
    }

    fn get_current_timestamp() -> u64 {
        let start = SystemTime::now();
        let since_the_epoch = start
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards");
        since_the_epoch.as_secs()
    }
}
