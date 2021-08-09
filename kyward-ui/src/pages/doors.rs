use yew::prelude::*;
use ybc::TileCtx::{Ancestor, Child, Parent};
use ybc::TileSize::Four;
use super::super::models::door::Door;
use yew::format::{Json, Nothing};
use yew::services::fetch::{FetchService, Request, Response};

pub enum Msg {
    AddDoor(Door),
    SetNew(Vec<Door>),
    Refresh,
}

#[derive(Clone, Properties)]
pub struct Properties {
  pub token: String
}

pub struct Doors {
    link: ComponentLink<Self>,
    doors: Vec<Door>,
    props: Properties,
}

impl Component for Doors {
    type Message = Msg;
    type Properties = Properties;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link,
            doors: vec![],
            props: props,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::AddDoor(new_door) => {
              // TODO: send to API
              
              self.doors.push(new_door);
              true
            }
            Msg::SetNew(new_doors) => {
              self.doors = new_doors.clone();
              true
            }
            Msg::Refresh => {
              self.get();
              false
            }
        }
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
            <ybc::Container fluid=true>
              <ybc::Tile ctx=Ancestor>
                <ybc::Tile ctx=Parent vertical=true size=Four>
                  <ybc::Tile ctx=Child classes=classes!("box")>
                    <ul>
                      {
                        self.doors.iter().map( |door|
                            html!{<p>{ &door.name }</p>}
                        ).collect::<Html>()
                      }
                    </ul>
                    <a href={ "/" }>{"Self"}</a>
                  </ybc::Tile>
                </ybc::Tile>
              </ybc::Tile>
            </ybc::Container>
        }
    }
}

impl Doors {
  fn get(&mut self) -> () {
    let req = Request::get("http://localhost:8080/api/doors")
      .header("Authentication", format!("Bearer {0}", self.props.token))
      .body(Nothing)
      .expect("Request error");
    let callback = self.link.callback(|response: Response<Json<Vec<Door>>>| {
        let Json(data) = response.into_body();
        Msg::SetNew(data)
    });
    let _task = FetchService::fetch(req, callback).expect("failed to start request");
  }
}