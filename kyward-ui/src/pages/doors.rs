use super::super::models::door::Door;
use super::super::utils::new_hero;
use ybc::TileCtx::{Child, Parent};
use yew::format::{Json, Nothing};
use yew::prelude::*;
use yew::services::fetch::FetchService;
use yew::services::fetch::{FetchTask, Request, Response};
use yew::services::ConsoleService;
use yew::web_sys;

pub enum Msg {
    GetResp(Result<Vec<Door>, anyhow::Error>),
    Refresh,
    Add,
}

#[derive(Clone, Properties, PartialEq)]
pub struct Properties {
    pub token: String,
}

pub struct Doors {
    link: ComponentLink<Self>,
    doors: Vec<Door>,
    fetching: Option<FetchTask>,
    props: Properties,
}

impl Component for Doors {
    type Message = Msg;
    type Properties = Properties;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        return |doors: Doors| -> Doors {
            doors
                .link
                .callback(|_: Msg| Msg::Refresh)
                .emit(Msg::Refresh);
            doors
        }(Self {
            link,
            fetching: None,
            doors: vec![],
            props: props,
        });
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::GetResp(resp) => {
                self.doors = resp.expect("Test");
                true
            }
            Msg::Refresh => {
                let req = Request::get("/api/v1alpha1/door")
                    .header("Authorization", format!("Bearer {0}", self.props.token))
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
            Msg::Add => {
                let next = match self.doors.last() {
                    Some(n) => n.id + 1,
                    None => return false,
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
                    .set_pathname(format!("/door/add/{0}", next).as_str())
                {
                    Ok(_) => true,
                    Err(err) => {
                        ConsoleService::error(format!("An error occured: {:#?}", err).as_str());
                        false
                    }
                };
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
            {new_hero("Doors", "Manage doors accessible by a group.")}
            <ybc::Section>
              <ybc::Container fluid=true>
                <ybc::Tile> // ctx=Ancestor
                  <ybc::Tile ctx=Parent vertical=true>
                    <ybc::Tile ctx=Child classes=classes!("box")>
                      <div class="buttons">
                        <ybc::Button classes=classes!("is-primary") onclick=self.link.callback(|_| Msg::Add).clone() >
                          { "Add" }
                        </ybc::Button>
                        <ybc::Button classes=classes!("is-info", "is-outlined") onclick=self.link.callback(|_| Msg::Refresh).clone() >
                          { "refresh" }
                        </ybc::Button>
                      </div>
                      <ybc::Table classes=classes!("is-fullwidth")>
                        <thead>
                          <tr>
                            <th>{"Name"}</th>
                            <th>{"Compartment"}</th>
                            <th>{"Level"}</th>
                            <th>{"Building"}</th>
                          </tr>
                        </thead>
                        <tbody>
                        {
                          self.doors.iter().map( |door|
                            html!{
                              <tr>
                                <th><a href={format!("/door/{0}", &door.id)}>{ &door.name }</a></th>
                                <th>{ &door.compartment }</th>
                                <th>{ &door.level }</th>
                                <th>{ &door.building }</th>
                              </tr>
                            }
                          ).collect::<Html>()
                        }
                        </tbody>
                      </ybc::Table>
                    </ybc::Tile>
                  </ybc::Tile>
                </ybc::Tile>
              </ybc::Container>
            </ybc::Section>
          </>
        }
    }
}
