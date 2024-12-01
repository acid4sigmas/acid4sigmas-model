use super::jwt::{Claim, JwtToken, UserClaims};
use super::ws::WsClient;
use crate::db::TableModel;
use crate::models::auth::AuthTokens;
use crate::models::db::{DatabaseAction, DatabaseRequest, DatabaseResponse, Filters, WhereClause};
use crate::to_string_;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;

#[async_trait::async_trait]
pub trait TokenHandler<T: Claim> {
    async fn new(secret_key: &str, client: Arc<Mutex<WsClient>>) -> Self
    where
        Self: Sized;
    async fn generate_token(&mut self, uid: i64, expires_in: usize) -> anyhow::Result<String>;
    async fn verify_token(&mut self, token: &str) -> Result<T, (String, u16)>;
}

use anyhow::{anyhow, Result};

pub struct UserTokenHandler {
    pub jwt: JwtToken,
    pub ws_client: Arc<Mutex<WsClient>>, // Use Arc<Mutex<WsClient>> instead of MutexGuard
}

#[async_trait::async_trait]
impl TokenHandler<UserClaims> for UserTokenHandler {
    async fn new(secret_key: &str, client: Arc<Mutex<WsClient>>) -> Self {
        Self {
            jwt: JwtToken::new(secret_key),
            ws_client: client,
        }
    }

    async fn generate_token(&mut self, uid: i64, expires_in: usize) -> Result<String> {
        let jti = uuid::Uuid::new_v4().to_string();
        let exp = (JwtToken::get_current_timestamp() as usize) + expires_in;

        let claims = UserClaims {
            user_id: uid.to_string(),
            exp,
            jti: jti.clone(), // jti = json token identifier
        };

        let db_request = DatabaseRequest {
            table: to_string_!("auth_tokens"),
            action: DatabaseAction::Insert,
            values: Some(
                AuthTokens {
                    jti,
                    uid,
                    expires_at: exp as i64,
                }
                .as_hash_map(),
            ),
            ..Default::default()
        };

        {
            let mut client = self.ws_client.lock().await;
            client
                .send(&db_request.to_string().map_err(|e| anyhow!(e))?)
                .await
                .map_err(|e| anyhow!(e))?;
        }

        let message = {
            let mut client = self.ws_client.lock().await;
            client.receive().await
        };

        // we are creating anytime a new lock and drop it immediately if not needed anymore to avoid locking delays
        // this will keep locking delays as short as possible

        if let Some(message) = message {
            let db_response = DatabaseResponse::<AuthTokens>::parse(&message.to_string())
                .map_err(|e| anyhow!(e))?;

            if db_response.is_error() {
                return Err(anyhow!("{}", db_response.error_message().unwrap()));
            }

            // we dont need to handle now anything else
            // because at this point we can only receive a Success response.
        }

        self.jwt.create_jwt(&claims).map_err(Into::into)
    }

    async fn verify_token(&mut self, token: &str) -> Result<UserClaims, (String, u16)> {
        let decoded_claims = self
            .jwt
            .decode_jwt::<UserClaims>(token)
            .map_err(|e| (e.to_string(), 401))?;

        let user_id_hashmap = {
            let mut hashmap = HashMap::new();
            let uid_i64 = decoded_claims
                .user_id
                .parse::<i64>()
                .map_err(|e| (e.to_string(), 500))?;
            hashmap.insert(to_string_!("uid"), serde_json::json!(uid_i64));
            hashmap
        };

        let db_request = DatabaseRequest {
            table: to_string_!("auth_tokens"),
            action: DatabaseAction::Retrieve,
            filters: Some(Filters {
                where_clause: Some(WhereClause::Single(user_id_hashmap)),
                ..Default::default()
            }),
            ..Default::default()
        };

        {
            let mut client = self.ws_client.lock().await;
            client
                .send(&db_request.to_string().map_err(|e| (e.to_string(), 500))?)
                .await
                .map_err(|e| (e.to_string(), 500))?;
        } // Release the lock explicitly here so other parts of the application can use the lock

        let message = {
            let mut client = self.ws_client.lock().await;
            client.receive().await
        };

        if let Some(message) = message {
            let db_response = DatabaseResponse::<AuthTokens>::parse(&message.to_string())
                .map_err(|e| (e.to_string(), 500))?;

            if db_response.is_error() {
                return Err((format!("{}", db_response.error_message().unwrap()), 500));
            }

            match db_response {
                DatabaseResponse::Data(token_props) => {
                    for token_prop in token_props {
                        if token_prop.jti == decoded_claims.jti {
                            return Ok(decoded_claims);
                        }
                    }

                    return Err((
                        to_string_!(
                            "token rejected. could not verify the trustworthiness of this token"
                        ),
                        401,
                    ));
                }
                _ => return Err((to_string_!("an unknown database error occurred."), 500)),
            }
        } else {
            return Err((to_string_!("Database response failed"), 500));
        }
    }
}
