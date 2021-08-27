use anyhow::Error;
use jwtk;
use rocket::http::Status;
use rocket::request::{self, FromRequest, Request};
use rocket::serde::Deserialize;
use async_std::task;

static AUTHENTICATION_HEADER: &'static str = "Authorization";
static BEARER_PATTERN: &'static str =
    r"Bearer ([a-zA-Z0-9_=]+\.[a-zA-Z0-9_=]+\.[a-zA-Z0-9_\-\+/=]*)";

#[derive(Debug, Clone, PartialEq)]
pub struct ApiToken(String);

impl ApiToken {
    pub fn validate(&self, validation_keys: &AzurePublicKeys) -> Result<User, AuthenticationError> {
        let kid = match match jwtk::decode_without_verify::<User>(self.0.as_str()) {
            Ok(token) => token,
            Err(err) => return Err(AuthenticationError(anyhow::Error::new(err))),
        }.header().kid.clone() {
            Some(kid) => kid,
            None => return Err(AuthenticationError(anyhow::Error::msg("Error: No kid in token"))),
        };
        let key = match validation_keys
            .clone()
            .keys
            .into_iter()
            .filter(|key| { 
                key.kid == kid 
            })
            .last() {
            Some(key) => key,
            None => return Err(AuthenticationError(anyhow::Error::msg("Error: No kid matched token"))),
        };
        let pem = format!(
                "-----BEGIN PUBLIC KEY-----\n{0}\n-----END PUBLIC KEY-----",
                key.x5c.last().unwrap()
        );
        println!("{0}", pem);
        let pem_bin = pem
            .as_str()
            .as_bytes();
        let validation_key = jwtk::SomePublicKey::from_pem(pem_bin.as_ref())
            .expect("Error: Wrong public key format");
        match jwtk::verify::<User>(self.0.as_str(), &validation_key) {
            Ok(token) => {
                let user: User = token.claims().extra.clone();
                Ok(user)
            }
            Err(err) => Err(AuthenticationError(anyhow::Error::new(err))),
        }
    }
}

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
        return request::Outcome::Success(ApiToken(token.to_string()));
    }
}


#[derive(Deserialize, Clone, PartialEq, Debug)]
pub struct User {
    name: String,
    unique_name: String,
    family_name: String,
    given_name: String,
    appid: String,
    ipaddr: String,
    idtyp: String,
    tenant_region_scope: String,
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for User {
    type Error = AuthenticationError;

    async fn from_request(req: &'r Request<'_>) -> request::Outcome<Self, Self::Error> {
        let api_token: ApiToken = match ApiToken::from_request(req)
            .await
            .success_or("Error: No token") {
                Ok(token) => token,
                Err(err) => {
                    return request::Outcome::Failure((
                        Status::Unauthorized,
                        AuthenticationError(anyhow::Error::msg(err)),
                    )) 
                }
        };
        let public_keys = match req.rocket().state::<AzurePublicKeys>() {
            Some(keys) => keys,
            None => {
                return request::Outcome::Failure((
                    Status::Unauthorized,
                    AuthenticationError(anyhow::Error::msg("Error: No public key in state!")),
                )) 
            }
        };
        let user = match api_token.validate(public_keys) {
            Ok(user) => user,
            Err(err) => {
                return request::Outcome::Failure((
                    Status::Unauthorized,
                    err,
                ))
            }
        };
        return request::Outcome::Success(user);
    }
}

#[derive(Deserialize, PartialEq, Clone, Debug)]
pub struct AzurePublicKey {
    kty: String,
    kid: String,
    x5t: String,
    n: String,
    e: String,
    x5c: Vec<String>,
}

#[derive(Deserialize, PartialEq, Clone, Debug)]
pub struct AzurePublicKeys {
    keys: Vec<AzurePublicKey>
}

pub fn get_oauth_public_key(url: &'static str) -> AzurePublicKeys {
    async fn get_closure(url: &'static str) -> AzurePublicKeys {
        let keys: AzurePublicKeys = reqwest::get(url)
            .await
            .unwrap()
            .json()
            .await
            .expect("test2");
        keys
    }
    task::block_on(get_closure(url))
}
