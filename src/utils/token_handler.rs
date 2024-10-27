use std::collections::HashMap;

use super::jwt::{Claim, JwtToken, UserClaims};
use super::ws::WsClient;
use crate::db::TableModel;
use crate::models::auth::AuthTokens;
use crate::models::db::{DatabaseAction, DatabaseRequest, DatabaseResponse, Filters, WhereClause};
use crate::to_string_;

#[async_trait::async_trait]
pub trait TokenHandler<'a, T: Claim> {
    async fn new(
        secret_key: &str,
        expires_in: usize,
        client: tokio::sync::MutexGuard<'a, WsClient>,
    ) -> Self
    where
        Self: Sized;
    async fn generate_token(&mut self, uid: i64) -> anyhow::Result<String>;
    async fn verify_token(&mut self, token: &str) -> anyhow::Result<T>;
}

use anyhow::{anyhow, Result};
use tokio::sync::MutexGuard;

pub struct UserTokenHandler<'a> {
    pub jwt: JwtToken,
    pub expires_in: usize,
    pub ws_client: MutexGuard<'a, WsClient>,
}

#[async_trait::async_trait]
impl<'a> TokenHandler<'a, UserClaims> for UserTokenHandler<'a> {
    async fn new(secret_key: &str, expires_in: usize, client: MutexGuard<'a, WsClient>) -> Self {
        Self {
            jwt: JwtToken::new(secret_key),
            expires_in,
            ws_client: client,
        }
    }

    async fn generate_token(&mut self, uid: i64) -> Result<String> {
        let jti = uuid::Uuid::new_v4().to_string();
        let exp = (JwtToken::get_current_timestamp() as usize) + self.expires_in;

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

        let client = &mut self.ws_client;

        client
            .send(&db_request.to_string().map_err(|e| anyhow!(e))?)
            .await
            .map_err(|e| anyhow!(e))?;

        if let Some(message) = client.receive().await {
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

    async fn verify_token(&mut self, token: &str) -> anyhow::Result<UserClaims> {
        let decoded_claims = self
            .jwt
            .decode_jwt::<UserClaims>(token)
            .map_err(|e| anyhow!(e))?;

        let user_id_hashmap = {
            let mut hashmap = HashMap::new();
            let uid_i64 = decoded_claims
                .user_id
                .parse::<i64>()
                .map_err(|e| anyhow!(e))?;
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

        let client = &mut self.ws_client;

        client
            .send(&db_request.to_string().map_err(|e| anyhow!(e))?)
            .await
            .map_err(|e| anyhow!(e))?;

        if let Some(message) = client.receive().await {
            let db_response = DatabaseResponse::<AuthTokens>::parse(&message.to_string())
                .map_err(|e| anyhow!(e))?;

            if db_response.is_error() {
                return Err(anyhow!("{}", db_response.error_message().unwrap()));
            }

            match db_response {
                DatabaseResponse::Data(token_props) => {
                    // we compare our token identifiers from the Database
                    // with the token identifiers from the claims
                    // if the token identifier matches with one of the claims
                    // we know that this token has been signed by our backend and is trustworthy

                    for token_prop in token_props {
                        if token_prop.jti == decoded_claims.jti {
                            return Ok(decoded_claims); // if the jti matches, we know this is a trustworthy token, therefore, we return Ok with the claims
                        }
                    }

                    return Err(anyhow!(
                        "token rejected. could not verify the trustworthiness of this token"
                    ));
                }
                _ => return Err(anyhow!("an unknown database error occurred.")), // if this gets called, make sure to check your previous code, the db-api will not return anything else except an error or the data even if they are empty.
            }
        } else {
            return Err(anyhow!("Database response failed"));
        }
    }
}