// use super::super::super::router::KywardRouter;
use yew::services::ConsoleService;
use yew::web_sys;
use anyhow;
use oauth2::{
  // reqwest::http_client,
  basic::BasicClient,
  url::Url,
  AuthUrl, AuthorizationCode, ClientId, ClientSecret, CsrfToken, PkceCodeChallenge, RedirectUrl,
  Scope, TokenResponse, TokenUrl,
};
use ybc::TileSize::Four;
use yew::prelude::*;

pub enum Msg {
  Redirect
}

#[derive(Clone, PartialEq)]
pub struct OauthConfig {
  pub client_id: String,
  pub auth_url: String,
  pub token_url: String,
  pub redirect_url: String,
  pub client_secret: String,
}

#[derive(Clone, Properties, PartialEq)]
pub struct Properties {}

pub struct Login {
  // `ComponentLink` is like a reference to a component.
  // It can be used to send messages to the component
  props: Properties,
  oauth: OauthConfig,
  link: ComponentLink<Self>,
}

impl Component for Login {
  type Message = Msg;
  type Properties = Properties;

  fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
    Self { 
      link: link,
      props: props,
      oauth: OauthConfig {
        client_id: "client_id".to_string(),
        client_secret: "client_secret".to_string(),
        auth_url: "http://authorize".to_string(),
        token_url: "http://token".to_string(),
        redirect_url:"http://redirect".to_string(),
      }
     }
  }

  fn update(&mut self, msg: Self::Message) -> ShouldRender {
    match msg {
      Msg::Redirect => {
        let (redirect_url, _csrf_token) = match get_redirect_url(&self.oauth) {
          Ok(res) => res,
          Err(err) => { 
            ConsoleService::error(format!("An error occured: {:#?}", err).as_str());
            return false 
          },
        };
        let window: web_sys::Window = match web_sys::window() {
          Some(window) => window,
          None => {
              ConsoleService::warn("No window to catch by websys!");
              return false;
          }
        };
        return match window
          .location()
          .set_href(redirect_url.as_str())
        {
          Ok(_) => true,
          Err(err) => {
              ConsoleService::error(format!("An error occured: {:#?}", err).as_str());
              false
          }
        }
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


fn get_redirect_url(oauth_config: &OauthConfig) -> Result<(Url, CsrfToken), anyhow::Error> {
    let client = BasicClient::new(
      ClientId::new(oauth_config.client_id.clone()),
      Some(ClientSecret::new(oauth_config.client_secret.clone())),
      AuthUrl::new(oauth_config.auth_url.clone())?,
      Some(TokenUrl::new(oauth_config.token_url.clone())?),
    )
    // Set the URL the user will be redirected to after the authorization process.
    .set_redirect_uri(RedirectUrl::new(oauth_config.redirect_url.clone())?);

    // Generate a PKCE challenge.
    let (pkce_challenge, _pkce_verifier) = PkceCodeChallenge::new_random_sha256();

    // Generate the full authorization URL.
    let (auth_url, csrf_token) = client
      .authorize_url(CsrfToken::new_random)
      // Set the desired scopes.
      .add_scope(Scope::new("read".to_string()))
      .add_scope(Scope::new("write".to_string()))
      // Set the PKCE code challenge.
      .set_pkce_challenge(pkce_challenge)
      .url();
    Ok((auth_url, csrf_token))
}