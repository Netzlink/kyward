use anyhow::Error;
use rocket::http::Status;
use rocket::request::{self, FromRequest, Request};
use azure_jwt::*;
/// Auth header key
const AUTHENTICATION_HEADER: &'static str = "Authorization";
/// REGEX for an JWT Auth header
const BEARER_PATTERN: &'static str =
    r"Bearer ([a-zA-Z0-9_=]+\.[a-zA-Z0-9_=]+\.[a-zA-Z0-9_\-\+/=]+)";

/// Struct representing an ApiToken
#[derive(Debug, Clone, PartialEq)]
pub struct ApiToken(String);

impl ApiToken {
    //! Validate a ApiToken with the jwtk certificates
    pub async fn validate(&self) -> Result<User, AuthenticationError> {
        let token = self.clone();
        tokio::task::spawn_blocking(move || {
            let mut az_auth = AzureAuth::new("0d73fe1d-c27c-410b-bf83-1e12d82627fe").unwrap();
            match az_auth.validate_token(token.0.as_str()) {
                Ok(token) => {
                    Ok(User(token.claims))
                }
                Err(err) => Err(AuthenticationError(anyhow::Error::new(err))),
            }
        }).await.expect("Task panicked")
    }
}

/// Struct representing an error in auth
#[derive(Debug)]
pub struct AuthenticationError(Error);

/// Get ApiToken from a Request
#[rocket::async_trait]
impl<'r> FromRequest<'r> for ApiToken {
    type Error = AuthenticationError;

    async fn from_request(req: &'r Request<'_>) -> request::Outcome<Self, Self::Error> {
        let map = req.headers().clone();
        let token_header = match map.get_one(AUTHENTICATION_HEADER) {
            Some(token_header) => token_header,
            None => {
                return request::Outcome::Failure((
                    Status::Unauthorized,
                    AuthenticationError(anyhow::Error::msg("No Authentication-header found!")),
                ))
            }
        };
        let token = &match regex::Regex::new(BEARER_PATTERN)
            .unwrap()
            .captures(token_header)
        {
            Some(token_string) => token_string,
            None => {
                return request::Outcome::Failure((
                    Status::Unauthorized,
                    AuthenticationError(anyhow::Error::msg("Bad Authentication-header")),
                ))
            }
        }[1];
        return request::Outcome::Success(ApiToken(token.to_string()));
    }
}

/// Microsoft Azure JWT Claims representing user-data
#[derive(Debug)]
pub struct User(azure_jwt::AzureJwtClaims);

/// Returns User from a Request
#[rocket::async_trait]
impl<'r> FromRequest<'r> for User {
    type Error = AuthenticationError;

    async fn from_request(req: &'r Request<'_>) -> request::Outcome<Self, Self::Error> {
        let api_token: ApiToken = match ApiToken::from_request(req)
            .await
            .success_or("Error: No token")
        {
            Ok(token) => token,
            Err(err) => {
                return request::Outcome::Failure((
                    Status::Unauthorized,
                    AuthenticationError(anyhow::Error::msg(err)),
                ))
            }
        };
        let user = match api_token.validate().await {
            Ok(user) => user,
            Err(err) => return request::Outcome::Failure((Status::Unauthorized, err)),
        };
        let _ = api_token;
        return request::Outcome::Success(user);
    }
}