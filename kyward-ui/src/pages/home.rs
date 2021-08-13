use super::super::utils::new_hero;
use yew::prelude::*;

pub enum Msg {}

pub struct Home {
    _link: ComponentLink<Self>,
}

impl Component for Home {
    type Message = Msg;
    type Properties = ();

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self { _link: link }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        false
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        // Should only return "true" if new properties are different to
        // previously received properties.
        // This component has no properties so we will always return "false".
        false
    }

    fn view(&self) -> Html {
        // https://bulma.io/documentation/overview/start/
        html! {
            <>
              {new_hero("Home", "a nice place to be.")}
              <ybc::Container fluid=true>
              <ybc::Section classes=classes!("is-large")>
                <ybc::Title>{"Hello!"}</ybc::Title>
                <ybc::Title classes=classes!("subtitle")>{"Welcome to Kyward."}</ybc::Title>
              </ybc::Section>
            </ybc::Container>
            </>
        }
    }
}
