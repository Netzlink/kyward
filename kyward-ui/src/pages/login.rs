// use super::super::super::router::KywardRouter;
use anyhow;
use async_std::task;
use chrono::{prelude::*, Duration};
use oauth2::reqwest::async_http_client;
use oauth2::{
    basic::BasicClient, url::Url, AuthType, AuthUrl, AuthorizationCode, ClientId, CsrfToken,
    PkceCodeChallenge, PkceCodeVerifier, RedirectUrl, Scope, TokenUrl,
};
use regex::Regex;
use serde_json;
use wasm_cookies;
use ybc::TileSize::Four;
use yew::prelude::*;
use yew::services::ConsoleService;
use yew::web_sys;

type TokenResponse =
    oauth2::StandardTokenResponse<oauth2::EmptyExtraTokenFields, oauth2::basic::BasicTokenType>;

pub enum Msg {
    Redirect,
}

#[derive(Clone, PartialEq)]
pub struct OauthConfig {
    pub client_id: String,
    pub auth_url: String,
    pub token_url: String,
    pub redirect_url: String,
}

#[derive(Clone, Properties, PartialEq)]
pub struct Properties {}

pub struct Login {
    props: Properties,
    oauth: OauthConfig,
    link: ComponentLink<Self>,
}

impl Component for Login {
    type Message = Msg;
    type Properties = Properties;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let login = Self {
            link: link,
            props: props,
            oauth: OauthConfig {
                client_id: "0d73fe1d-c27c-410b-bf83-1e12d82627fe"
                    .to_string(),
                auth_url: "".to_string(),
                token_url: "".to_string(),
                redirect_url: "".to_string(),
            },
        };
        let window: web_sys::Window = match web_sys::window() {
            Some(window) => window,
            None => {
                ConsoleService::error("No window to catch by websys!");
                panic!("No window to catch by websys!")
            }
        };
        match match window.location().pathname() {
            Ok(pathname) => pathname,
            Err(err) => {
                ConsoleService::error(format!("Error location: {:#?}", err).as_str());
                panic!("Error location.")
            }
        }
        .as_str()
        {
            "/auth/callback" => {
                let oauth = login.oauth.clone();
                let cookie_opts = wasm_cookies::CookieOptions {
                    path: Some("/"),
                    domain: None,
                    expires: Some((Local::now() + Duration::minutes(30)).to_string()),
                    same_site: wasm_cookies::SameSite::Strict,
                    secure: false,
                };
                let get_token_future = async move {
                    let token = get_token(&oauth, window.clone()).await.unwrap();
                    ConsoleService::info(format!("Token: {:#?}", token).as_str());
                    wasm_cookies::set(
                        "token",
                        match serde_json::to_string(&token) {
                            Ok(json) => json,
                            Err(err) => {
                                ConsoleService::error(format!("An error occured: {:#?}", err).as_str());
                                panic!("An error occured: {:#?}", err)
                            }
                        }
                        .as_str(),
                        &cookie_opts,
                    );
                    match window.location().set_href("/") {
                        Ok(_) => {},
                        Err(err) => {
                            ConsoleService::error(format!("Error: {:#?}", err).as_str());
                        },
                    };
                };
                task::block_on(get_token_future);
            }
            _ => {}
        };
        return login;
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Redirect => {
                let (redirect_url, csrf_token, pkce_verifier) = match get_redirect_url(&self.oauth)
                {
                    Ok(res) => res,
                    Err(err) => {
                        ConsoleService::error(format!("An error occured: {:#?}", err).as_str());
                        return false;
                    }
                };
                let cookie_opts = wasm_cookies::CookieOptions {
                    path: Some("/"),
                    domain: None,
                    expires: Some((Local::now() + Duration::minutes(1)).to_string()),
                    same_site: wasm_cookies::SameSite::Strict,
                    secure: false,
                };
                wasm_cookies::set(
                    "csrf_token",
                    match serde_json::to_string(&csrf_token) {
                        Ok(json) => json,
                        Err(err) => {
                            ConsoleService::error(format!("An error occured: {:#?}", err).as_str());
                            panic!("An error occured: {:#?}", err)
                        }
                    }
                    .as_str(),
                    &cookie_opts,
                );
                wasm_cookies::set(
                    "pkce_verifier",
                    match serde_json::to_string(&pkce_verifier) {
                        Ok(json) => json,
                        Err(err) => {
                            ConsoleService::error(format!("An error occured: {:#?}", err).as_str());
                            panic!("An error occured: {:#?}", err)
                        }
                    }
                    .as_str(),
                    &cookie_opts,
                );
                let window: web_sys::Window = match web_sys::window() {
                    Some(window) => window,
                    None => {
                        ConsoleService::warn("No window to catch by websys!");
                        return false;
                    }
                };
                return match window.location().set_href(redirect_url.as_str()) {
                    Ok(_) => true,
                    Err(err) => {
                        ConsoleService::error(format!("An error occured: {:#?}", err).as_str());
                        false
                    }
                };
            }
        }
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.props != props
    }

    fn view(&self) -> Html {
        // https://bulma.io/documentation/overview/start/
        html! {
          <>
            <section class=classes!{"section", "is-large"}>
              <ybc::Tile size=Four vertical=true classes=classes!{"box"}>
                <ybc::Title>
                {"Login"}
                </ybc::Title>
                {"with Microsoft Azure"}
                <hr/>
                <ybc::Button
                  classes=classes!{"is-primary"}
                  onclick=self.link.callback(|_| {
                    Msg::Redirect
                  })
                >
                  {"Login"}
                </ybc::Button>
              </ybc::Tile>
            </section>
          </>
        }
    }
}

fn get_oauth_client(oauth_config: &OauthConfig) -> Result<BasicClient, anyhow::Error> {
    let client = BasicClient::new(
        ClientId::new(oauth_config.client_id.clone()),
        None,
        AuthUrl::new(oauth_config.auth_url.clone())?,
        Some(TokenUrl::new(oauth_config.token_url.clone())?),
    )
    .set_auth_type(AuthType::RequestBody)
    .set_redirect_uri(RedirectUrl::new(oauth_config.redirect_url.clone())?);
    Ok(client)
}

fn get_redirect_url(
    oauth_config: &OauthConfig,
) -> Result<(Url, CsrfToken, PkceCodeVerifier), anyhow::Error> {
    // Generate a PKCE challenge.
    let (pkce_challenge, pkce_verifier) = PkceCodeChallenge::new_random_sha256();

    // Generate the full authorization URL.
    let (auth_url, csrf_token) = get_oauth_client(oauth_config)?
        .authorize_url(CsrfToken::new_random)
        // Set the desired scopes.
        .add_scope(Scope::new("openid".to_string()))
        .add_scope(Scope::new("profile".to_string()))
        // Set the PKCE code challenge.
        .set_pkce_challenge(pkce_challenge)
        .url();
    Ok((auth_url, csrf_token, pkce_verifier))
}

async fn get_token(
    oauth_config: &OauthConfig,
    window: web_sys::Window,
) -> Result<TokenResponse, anyhow::Error> {
    let href = match window.location().href() {
        Ok(href) => href,
        Err(err) => {
            ConsoleService::error(format!("Error location: {:#?}", err).as_str());
            return Err(anyhow::Error::msg(format!("Error: {:#?}", err)));
        }
    };
    let code = match Regex::new(r"code=([\w,\.,\-]+)")
        .unwrap()
        .captures(href.as_str())
    {
        Some(captures) => captures,
        None => {
            ConsoleService::error("Error regex code");
            return Err(anyhow::Error::msg("Error regex code".to_string()));
        }
    }[1]
    .to_string();
    let state = match Regex::new(r"&state=([\w,\.,\-]+)")
        .unwrap()
        .captures(href.as_str())
    {
        Some(captures) => captures,
        None => {
            ConsoleService::error("Error regex code");
            return Err(anyhow::Error::msg("Error regex code".to_string()));
        }
    }[1]
    .to_string();
    ConsoleService::info(format!("Code: {0}, State: {1}", code, state).as_str());

    let pkce_verifier_json = match match wasm_cookies::get("pkce_verifier") {
        Some(pkce_verifier_result) => pkce_verifier_result,
        None => {
            ConsoleService::error("Error cookie 'pkce_verifier' not found");
            return Err(anyhow::Error::msg(
                "Error cookie 'pkce_verifier' not found".to_string(),
            ));
        }
    } {
        Ok(pkce_verifier) => pkce_verifier,
        Err(err) => {
            ConsoleService::error(format!("Error: {:#?}", err).as_str());
            return Err(anyhow::Error::new(err));
        }
    };
    let pkce_verifier: PkceCodeVerifier = match serde_json::from_str(pkce_verifier_json.as_str()) {
        Ok(pkce_verifier) => pkce_verifier,
        Err(err) => {
            ConsoleService::error(format!("Error: {:#?}", err).as_str());
            return Err(anyhow::Error::new(err));
        }
    };
    let token_result = get_oauth_client(oauth_config)?
        .exchange_code(AuthorizationCode::new(code))
        .set_pkce_verifier(pkce_verifier)
        .request_async(async_http_client)
        .await?;
    Ok(token_result)
}
