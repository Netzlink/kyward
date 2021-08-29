use wasm_cookies;
use yew::prelude::*;
use yew::services::ConsoleService;
use yew::web_sys;

pub enum Msg {}

#[derive(Clone, Properties, PartialEq)]
pub struct Properties {}

pub struct Logout {
    _props: Properties,
    _link: ComponentLink<Self>,
}

impl Component for Logout {
    type Message = Msg;
    type Properties = Properties;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        for cookie in vec!["csrf_token", "pkce_verifier", "token"] {
            wasm_cookies::delete(cookie);
        }
        let window: web_sys::Window = match web_sys::window() {
            Some(window) => window,
            None => {
                ConsoleService::error("No window to catch by websys!");
                panic!("No window to catch by websys!")
            }
        };
        window.location().set_href("/").unwrap();
        Self {
            _link: link,
            _props: props,
        }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        true
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        true
    }

    fn view(&self) -> Html {
        html! {}
    }
}
