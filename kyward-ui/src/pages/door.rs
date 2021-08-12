use super::super::models::door::Door;
use super::super::utils::new_hero;
use serde_json::json;
use ybc::TileCtx::{Child, Parent};
use yew::format::{Json, Nothing};
use yew::prelude::*;
use yew::services::fetch::FetchService;
use yew::services::fetch::{FetchTask, Request, Response};

pub enum Action {
  Name,
  Compartment,
  Level,
  Building,
  Description,
}

pub enum Msg {
  Add,
  Get,
  GetResp(Result<Vec<Door>, anyhow::Error>),
  Update,
  Delete,
  Return,
  Set(Action, String),
  Nothing,
}

#[derive(Clone, Properties)]
pub struct Properties {
  pub token: String,
  pub id: i32,
}

pub struct DoorPage {
  link: ComponentLink<Self>,
  doors: Option<Vec<Door>>,
  fetching: Option<FetchTask>,
  props: Properties,
}

impl Component for DoorPage {
  type Message = Msg;
  type Properties = Properties;

  fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
    return |doors: DoorPage| -> DoorPage {
      doors.link.callback(|_: Msg| Msg::Get).emit(Msg::Get);
      doors
    }(Self {
      link,
      fetching: None,
      doors: None,
      props: props,
    });
  }

  fn update(&mut self, msg: Self::Message) -> ShouldRender {
    match msg {
      Msg::GetResp(resp) => {
        self.doors = match resp {
          Ok(doors) => Some(doors),
          _ => None,
        };
        true
      }
      Msg::Get => {
        let req = Request::get(format!(
          "http://localhost:8000/api/v1alpha1/door/{0}",
          self.props.id
        ))
        .body(Nothing)
        .expect("can make req to jsonplaceholder");

        let cb = self.link.callback(
          |response: Response<Json<Result<Vec<Door>, anyhow::Error>>>| {
            let Json(data) = response.into_body();
            Msg::GetResp(data)
          },
        );

        let task = FetchService::fetch(req, cb).expect("can create task");
        self.fetching = Some(task);
        true
      }
      Msg::Delete => {
        let req = Request::delete(format!(
          "http://localhost:8000/api/v1alpha1/door/{0}",
          self.props.id
        ))
        .body(Nothing)
        .expect("can make req to jsonplaceholder");

        let cb = self
          .link
          .callback(|_response: Response<Json<Result<i32, anyhow::Error>>>| Msg::Return);

        let task = FetchService::fetch(req, cb).expect("can create task");
        self.doors = None;
        self.fetching = Some(task);
        true
      }
      Msg::Update => {
        let door = &json!(self
          .doors
          .clone()
          .expect("aah")
          .first()
          .expect("Ahh")
          .clone());

        let req = Request::put("http://localhost:8000/api/v1alpha1/door")
          .header("Content-Type", "application/json")
          .body(Json(door))
          .expect("can make req to jsonplaceholder");

        let cb = self
          .link
          .callback(|_response: Response<Json<Result<i32, anyhow::Error>>>| Msg::Get);

        let task = FetchService::fetch(req, cb).expect("can create task");
        self.fetching = Some(task);
        true
      }
      Msg::Set(action, value) => {
        let mut door = match &self.doors {
          Some(doors) => match doors.first() {
            Some(door) => door,
            None => return false,
          },
          None => return false,
        }
        .clone();
        match action {
          Action::Name => door.name = value,
          Action::Compartment => door.compartment = value,
          Action::Building => door.building = value,
          Action::Level => door.level = value,
          Action::Description => door.description = value,
        }
        self.doors = Some(vec![door]);
        true
      }
      Msg::Return => true,
      Msg::Add => true,
      Msg::Nothing => false,
    }
  }

  fn change(&mut self, _props: Self::Properties) -> ShouldRender {
    false
  }

  fn view(&self) -> Html {
    // https://bulma.io/documentation/overview/start/
    html! {
      <>
        {new_hero("Door", "Manage a door lock.")}
        <ybc::Section>
          <ybc::Container fluid=true>
            <ybc::Tile> // ctx=Ancestor
              <ybc::Tile ctx=Parent vertical=true>
                <ybc::Tile ctx=Child classes=classes!("box")>
                  {
                    match &self.doors {
                      Some(doors) => {
                        let door = match doors.first() {
                          Some(door) => door,
                          None => panic!("Err"),
                        };
                        html!{
                          <>
                            <ybc::Block>
                              <ybc::Title>{"Name"}</ybc::Title>
                              <input
                                class=classes!("input")
                                type={"text"}
                                placeholder={"Name"}
                                value={ door.name.to_owned() }
                                onchange={self.link.callback(|event: ChangeData| {
                                  match event {
                                    ChangeData::Value(name) => Msg::Set(Action::Name, name),
                                    _ => Msg::Nothing,
                                  }
                                }).clone()}
                              />
                            </ybc::Block>
                            <ybc::Block>
                              <ybc::Title>{"Compartment"}</ybc::Title>
                              <input
                                class=classes!("input")
                                type={"text"}
                                placeholder={"Compartment"}
                                value={ door.compartment.to_owned() }
                                onchange={self.link.callback(|event: ChangeData| {
                                  match event {
                                    ChangeData::Value(name) => Msg::Set(Action::Compartment, name),
                                    _ => Msg::Nothing,
                                  }
                                }).clone()}
                              />
                            </ybc::Block>
                            <ybc::Block>
                              <ybc::Title>{"Level"}</ybc::Title>
                              <input
                                class=classes!("input")
                                type={"text"}
                                placeholder={"Level"}
                                value={ door.level.to_owned() }
                                onchange={self.link.callback(|event: ChangeData| {
                                  match event {
                                    ChangeData::Value(name) => Msg::Set(Action::Level, name),
                                    _ => Msg::Nothing,
                                  }
                                }).clone()}
                              />
                            </ybc::Block>
                            <ybc::Block>
                              <ybc::Title>{"Building"}</ybc::Title>
                              <input
                                class=classes!("input")
                                type={"text"}
                                placeholder={"Building"}
                                value={ door.building.to_owned() }
                                onchange={self.link.callback(|event: ChangeData| {
                                  match event {
                                    ChangeData::Value(name) => Msg::Set(Action::Building, name),
                                    _ => Msg::Nothing,
                                  }
                                }).clone()}
                              />
                            </ybc::Block>
                            <ybc::Block>
                              <ybc::Title>{"Description"}</ybc::Title>
                              <input
                                class=classes!("input")
                                type={"text"}
                                placeholder={"Description"}
                                value={ door.description.to_owned() }
                                onchange={self.link.callback(|event: ChangeData| {
                                  match event {
                                    ChangeData::Value(name) => Msg::Set(Action::Description, name),
                                    _ => Msg::Nothing,
                                  }
                                }).clone()}
                              />
                            </ybc::Block>
                            <hr/>
                            <div class={"columns"}>
                              <div class={"column"}>
                                <ybc::Button
                                  classes=classes!("is-primary")
                                  onclick={self.link.callback(|_| Msg::Update).clone()}
                                >
                                  { "Save" }
                                </ybc::Button>
                              </div>
                              <div class={"column"}>
                                <ybc::Button
                                  classes=classes!("is-danger")
                                  onclick={self.link.callback(|_| Msg::Delete).clone()}
                                >
                                  { "Delete" }
                                </ybc::Button>
                              </div>
                            </div>
                          </>
                        }
                      },
                      None => html!{
                        <ybc::Notification classes=classes!("is-danger")>
                          <ybc::Button classes=classes!("delete")/>
                          {"An error occurred. Dunno... Maybe check your Connection"}
                        </ybc::Notification>
                      }
                    }
                  }
                </ybc::Tile>
              </ybc::Tile>
            </ybc::Tile>
          </ybc::Container>
        </ybc::Section>
      </>
    }
  }
}
