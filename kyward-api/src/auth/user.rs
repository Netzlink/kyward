use rocket::http::Status;
use rocket::request::{self, FromRequest, Request};
use super::error::{AuthenticationError, NewAuthenticationError};
use super::token::ApiToken;

/// Microsoft Azure JWT Claims representing user-data
#[derive(Debug)]
pub struct User(pub azure_jwt::AzureJwtClaims);

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
                    NewAuthenticationError(anyhow::Error::msg(err)),
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