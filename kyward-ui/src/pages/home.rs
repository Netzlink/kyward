use super::super::models::user::User;
use super::super::utils::new_hero;
use base64;
use regex::Regex;
use std::str;
use yew::prelude::*;
use yew::services::ConsoleService;

pub enum Msg {
    GetUser,
}

#[derive(Properties, Clone, PartialEq)]
pub struct Properties {
    pub token: String,
}

pub struct Home {
    link: ComponentLink<Self>,
    user: Option<User>,
    props: Properties,
}

impl Component for Home {
    type Message = Msg;
    type Properties = Properties;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        return |s: Self| -> Self {
            s.link.callback(|s: Msg| s).emit(Msg::GetUser);
            s
        }(Self {
            link: link,
            user: None,
            props: props,
        });
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::GetUser => {
                let claims_json = match Regex::new(r"\.([a-zA-Z\d+/]+)\.")
                    .unwrap()
                    .captures(self.props.token.as_str())
                {
                    Some(json) => json,
                    None => {
                        ConsoleService::warn("Error: No valid JWT");
                        return false;
                    }
                }[1]
                .to_string();
                let bytes = match base64::decode(claims_json.as_str()) {
                    Ok(bytes) => bytes,
                    Err(err) => {
                        ConsoleService::warn(format!("Error: {:#?}", err).as_str());
                        return false;
                    }
                };
                let user: User =
                    match serde_json::from_str(match str::from_utf8(bytes.as_slice()) {
                        Ok(string) => string,
                        Err(err) => {
                            ConsoleService::warn(format!("Error: {:#?}", err).as_str());
                            return false;
                        }
                    }) {
                        Ok(user) => user,
                        Err(err) => {
                            ConsoleService::warn(format!("Error: {:#?}", err).as_str());
                            return false;
                        }
                    };
                self.user = Some(user);
                true
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
              {new_hero("Home", "a nice place to be.")}
              <ybc::Container fluid=true>
              <ybc::Section classes=classes!("is-large")>
                <ybc::Title>
                  {
                    format!("Hello, {0}!", match self.user.clone() {
                        Some(user) => user.given_name,
                        None => "Anon".to_string(),
                    })
                  }
                </ybc::Title>
                <ybc::Title classes=classes!("subtitle")>{"Welcome to Kyward."}</ybc::Title>
              </ybc::Section>
            </ybc::Container>
            </>
        }
    }
}
