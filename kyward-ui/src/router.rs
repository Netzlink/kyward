use yew::prelude::*;
use yew_router::prelude::*;

use super::pages::{home, doors};

#[derive(Switch, Debug, Clone)]
pub enum AppRoute {
    #[to = "/doors"]
    Doors,
    #[to = "/home"]
    Home,
    #[to = "/"]
    Index,
}

pub enum Msg {

}

pub struct KywardRouter {
    link: ComponentLink<Self>,
}

impl Component for KywardRouter {
    type Message = Msg;
    type Properties = ();

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link: link,
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

    fn view (&self) -> Html {
      html! {
        <Router<AppRoute, ()>
            render = Router::render(|switch: AppRoute| {
                match switch {
                    AppRoute::Index => html!{<home::Home/>},
                    AppRoute::Home => html!{<home::Home/>},
                    AppRoute::Doors => html!{<doors::Doors/>}
                }
            })
        />
      }
    }
}