use yew::prelude::*;
use yew_router::prelude::*;

use super::pages::{
    login,
    door, 
    doors,
    companies,
    home
};

#[derive(Switch, Debug, Clone)]
pub enum AppRoute {
    #[to = "/login"]
    Login,
    #[to = "/logout"]
    Logout,
    #[to = "/doors"]
    Doors,
    #[to = "/door/{id}"]
    Door(i32),
    #[to = "/door/add/{id}"]
    DoorAdd(i32),
    #[to = "/companies"]
    Companies,
    #[to = "/home"]
    Home,
    #[to = "/"]
    Index,
}

pub enum Msg {}

#[derive(Clone, Properties, PartialEq)]
pub struct Properties {
    pub token: String,
}

pub struct KywardRouter {
    _link: ComponentLink<Self>,
    props: Properties,
}

impl Component for KywardRouter {
    type Message = Msg;
    type Properties = Properties;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self { 
            _link: link,
            props: props,
        }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        true
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        // Should only return "true" if new properties are different to
        // previously received properties.
        // This component has no properties so we will always return "false".
        false
    }

    fn view(&self) -> Html {
        html! {
          <Router<AppRoute, ()>
              render = Router::render(|switch: AppRoute| {
                  match switch {
                      AppRoute::Index => html!{<home::Home/>},
                      AppRoute::Home => html!{<home::Home/>},
                      AppRoute::Login => html!{<login::Login/>},
                      AppRoute::Logout => html!{<>{"Not yet Implemented"}</>},
                      AppRoute::Doors => html!{
                        <doors::Doors 
                          token=""
                        />
                      },
                      AppRoute::Door(id) => html!{
                        <door::DoorPage
                          token=""
                          id=id 
                          add=false 
                        />
                      },
                      AppRoute::DoorAdd(id) => html!{
                        <door::DoorPage 
                          token=""
                          id=id 
                          add=true 
                        />
                      },
                      AppRoute::Companies => html!{
                        <companies::Companies 
                            token=""
                        />
                      }
                  }
              })
          />
        }
    }
}
