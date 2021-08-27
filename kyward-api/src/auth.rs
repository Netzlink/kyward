use anyhow::Error;
use rocket::http::Status;
use rocket::request::{self, FromRequest, Request};
use azure_jwt::*;

static AUTHENTICATION_HEADER: &'static str = "Authorization";
static BEARER_PATTERN: &'static str =
    r"Bearer ([a-zA-Z0-9_=]+\.[a-zA-Z0-9_=]+\.[a-zA-Z0-9_\-\+/=]*)";

#[derive(Debug, Clone, PartialEq)]
pub struct ApiToken(String);

#[derive(Debug)]
pub struct AuthenticationError(Error);

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
        let api_token = ApiToken(token.to_string());
        return request::Outcome::Success(api_token);
    }
}

impl ApiToken {
    pub fn _validate(&self, auth_conf: &OauthConf) -> Result<AzureJwtClaims, AuthenticationError> {
        let mut auth = match AzureAuth::new(auth_conf.app_id.clone()) {
            Ok(data) => data,
            Err(err) => {
                return Err(AuthenticationError(anyhow::Error::new(err)));
            }
        };
        match auth.validate_token(self.0.as_str()) {
            Ok(token) => Ok(token.claims),
            Err(err) => Err(AuthenticationError(anyhow::Error::new(err))),
        }
    }
}

pub struct OauthConf {
    pub app_id: String
}

pub fn get_azure_auth(id: &'static str) -> OauthConf {
    OauthConf{
        app_id: id.to_string(),
    }
}