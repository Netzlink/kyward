// use super::super::super::router::KywardRouter;
use yew::prelude::*;
use ybc::TileSize::Four;

pub enum Msg {}

pub struct Login {
    // `ComponentLink` is like a reference to a component.
    // It can be used to send messages to the component
    _link: ComponentLink<Self>,
}

impl Component for Login {
    type Message = Msg;
    type Properties = ();

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self { _link: link }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        true
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
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
                <ybc::Button classes=classes!{"is-primary"}>{"Login"}</ybc::Button>
              </ybc::Tile>
            </section>
          </>
        }
    }
}
