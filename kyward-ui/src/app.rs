use super::router::KywardRouter;
use yew::prelude::*;

pub enum Msg {}

pub struct App {
  // `ComponentLink` is like a reference to a component.
  // It can be used to send messages to the component
  link: ComponentLink<Self>,
}

impl Component for App {
  type Message = Msg;
  type Properties = ();

  fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
    Self { link }
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
    // https://bulma.io/documentation/overview/start/
    html! {
      <>
        <ybc::Navbar
          transparent=false
          padded=true
          spaced=true
          navbrand=html! {
            <a href={"/home"}>
              <ybc::Title>
                <ybc::Icon>
                  <i class=classes!{"fas", "fa-home"}></i>
                </ybc::Icon>
                {"Kyward"}
              </ybc::Title>
            </a>
          }
          navend=html! {
            <>
              <a class=classes!{"navbar-item"} href={"/doors"} >{"Doors"}</a>
            </>
          }
        />
        <KywardRouter/>
      </>
    }
  }
}
