use super::super::models::company::Company;
use super::super::utils::new_hero;
use serde_json::json;
use ybc::TileCtx::{Child, Parent};
use yew::format::{Json, Nothing};
use yew::prelude::*;
use yew::services::console::ConsoleService;
use yew::services::fetch::FetchService;
use yew::services::fetch::{FetchTask, Request, Response};
use yew::web_sys;

pub enum Action {
    Name,
    Description,
}

pub enum Msg {
    Add,
    Get,
    GetResp(Result<Vec<Company>, anyhow::Error>),
    Update,
    Delete,
    Return,
    Set(Action, String),
    Nothing,
}

#[derive(Clone, Properties, PartialEq)]
pub struct Properties {
    pub token: String,
    pub id: i32,
    pub add: bool,
}

pub struct CompanyPage {
    link: ComponentLink<Self>,
    companies: Option<Vec<Company>>,
    error: Option<anyhow::Error>,
    fetching: Option<FetchTask>,
    props: Properties,
}

impl Component for CompanyPage {
    type Message = Msg;
    type Properties = Properties;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        return |companies: CompanyPage| -> CompanyPage {
            if !companies.props.add {
                companies.link.callback(|_: Msg| Msg::Get).emit(Msg::Get)
            };
            companies
        }(Self {
            link,
            fetching: None,
            companies: match props.add {
                true => Some(vec![Company {
                    id: props.id,
                    name: "".to_string(),
                    description: "".to_string(),
                }]),
                false => None,
            },
            error: None,
            props: props,
        });
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::GetResp(resp) => {
                match resp {
                    Ok(companies) => {
                        self.companies = Some(companies);
                        self.error = None;
                    }
                    Err(err) => {
                        self.companies = None;
                        self.error = Some(err);
                    }
                };
                true
            }
            Msg::Get => {
                let req = match Request::get(format!("/api/v1alpha1/company/{0}", self.props.id))
                    .header("Authorization", format!("Bearer {0}", self.props.token))
                    .body(Nothing)
                {
                    Ok(req) => req,
                    Err(err) => {
                        self.error = Some(anyhow::Error::new(err));
                        return true;
                    }
                };

                let cb = self.link.callback(
                    |response: Response<Json<Result<Vec<Company>, anyhow::Error>>>| {
                        let Json(data) = response.into_body();
                        match data {
                            Ok(_) => return Msg::GetResp(data),
                            Err(err) => {
                                ConsoleService::info(format!("Error: {:?}", err).as_str());
                                return Msg::Nothing;
                            }
                        }
                    },
                );

                match FetchService::fetch(req, cb) {
                    Ok(task) => {
                        self.fetching = Some(task);
                    }
                    Err(err) => {
                        self.fetching = None;
                        self.error = Some(err);
                    }
                };
                true
            }
            Msg::Delete => {
                let req = match Request::delete(format!("/api/v1alpha1/company/{0}", self.props.id))
                    .header("Authorization", format!("Bearer {0}", self.props.token))
                    .body(Nothing)
                {
                    Ok(req) => req,
                    Err(err) => {
                        self.error = Some(anyhow::Error::new(err));
                        return true;
                    }
                };

                let cb =
                    self.link
                        .callback(|response: Response<Json<Result<i32, anyhow::Error>>>| {
                            let Json(data) = response.into_body();
                            match data {
                                Ok(_data) => {}
                                Err(err) => {
                                    ConsoleService::info(format!("Error: {:?}", err).as_str())
                                }
                            };
                            Msg::Return
                        });

                match FetchService::fetch(req, cb) {
                    Ok(task) => {
                        self.fetching = Some(task);
                        self.companies = None;
                    }
                    Err(err) => {
                        self.fetching = None;
                        self.error = Some(err);
                    }
                };
                true
            }
            Msg::Update => {
                let company = &json!(match match &self.companies {
                    Some(companies) => companies,
                    None => {
                        self.error = Some(anyhow::Error::msg("No door to update"));
                        return true;
                    }
                }
                .clone()
                .first()
                {
                    Some(company) => company,
                    None => {
                        self.error = Some(anyhow::Error::msg("No door fetched to update"));
                        return true;
                    }
                }
                .clone());

                let req = match Request::put("/api/v1alpha1/door")
                    .header("Authorization", format!("Bearer {0}", self.props.token))
                    .header("Content-Type", "application/json")
                    .body(Json(company))
                {
                    Ok(req) => req,
                    Err(err) => {
                        self.error = Some(anyhow::Error::new(err));
                        return true;
                    }
                };

                let cb =
                    self.link
                        .callback(|response: Response<Json<Result<i32, anyhow::Error>>>| {
                            let Json(data) = response.into_body();
                            match data {
                                Ok(_data) => {}
                                Err(err) => {
                                    ConsoleService::info(format!("Error: {:?}", err).as_str())
                                }
                            };
                            Msg::Get
                        });

                match FetchService::fetch(req, cb) {
                    Ok(task) => {
                        self.fetching = Some(task);
                    }
                    Err(err) => {
                        self.fetching = None;
                        self.error = Some(err);
                    }
                };
                true
            }
            Msg::Set(action, value) => {
                let mut company = match &self.companies {
                    Some(companies) => match companies.first() {
                        Some(company) => company,
                        None => return false,
                    },
                    None => return false,
                }
                .clone();
                match action {
                    Action::Name => company.name = value,
                    Action::Description => company.description = value,
                }
                self.companies = Some(vec![company]);
                true
            }
            Msg::Return => {
                let window: web_sys::Window = match web_sys::window() {
                    Some(window) => window,
                    None => {
                        ConsoleService::warn("No window to catch by websys!");
                        return false;
                    }
                };
                return match window.location().set_pathname("/companies") {
                    Ok(_) => true,
                    Err(err) => {
                        self.error = Some(anyhow::Error::msg(format!("Error: {:#?}", err)));
                        ConsoleService::error(format!("An error occured: {:#?}", err).as_str());
                        false
                    }
                };
            }
            Msg::Add => {
                let company = &json!(match match &self.companies {
                    Some(companies) => companies,
                    None => {
                        self.error = Some(anyhow::Error::msg("No door to add"));
                        return true;
                    }
                }
                .clone()
                .first()
                {
                    Some(company) => company,
                    None => {
                        self.error = Some(anyhow::Error::msg("No door fetched to add"));
                        return true;
                    }
                }
                .clone());

                let req = match Request::post("/api/v1alpha1/company")
                    .header("Authorization", format!("Bearer {0}", self.props.token))
                    .header("Content-Type", "application/json")
                    .body(Json(company))
                {
                    Ok(req) => req,
                    Err(err) => {
                        self.error = Some(anyhow::Error::new(err));
                        return true;
                    }
                };

                let cb =
                    self.link
                        .callback(|response: Response<Json<Result<i32, anyhow::Error>>>| {
                            let Json(data) = response.into_body();
                            match data {
                                Ok(_data) => {}
                                Err(err) => {
                                    ConsoleService::info(format!("Error: {:?}", err).as_str())
                                }
                            };
                            Msg::Get
                        });
                self.props.add = false;
                match FetchService::fetch(req, cb) {
                    Ok(task) => {
                        self.fetching = Some(task);
                    }
                    Err(err) => {
                        self.fetching = None;
                        self.error = Some(err);
                    }
                };
                true
            }
            Msg::Nothing => false,
        }
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.props != props
    }

    fn view(&self) -> Html {
        // https://bulma.io/documentation/overview/start/
        html! {
          <>
            {new_hero("Company", "Manage a company.")}
            <ybc::Section>
              <ybc::Container fluid=true>
                <ybc::Tile> // ctx=Ancestor
                  <ybc::Tile ctx=Parent vertical=true>
                    <ybc::Tile ctx=Child classes=classes!("box")>
                      {
                        match &self.error {
                          Some(err) => html!{
                            <>
                              <ybc::Notification classes=classes!("is-danger")>
                                <ybc::Title>
                                  {"Not Found"}
                                </ybc::Title>
                                {"An error occurred. Dunno... Maybe check your Connection"}
                                <pre>
                                  { format!("Error: {:#?}", err) }
                                </pre>
                              </ybc::Notification>
                              <a class={"button"} href={"/doors"} >{"Back"}</a>
                            </>
                          },
                          None => match &self.companies {
                            Some(companies) => {
                              let company = match companies.first() {
                                Some(company) => company.clone(),
                                None => {
                                    return html!{
                                      <>
                                        <ybc::Notification classes=classes!("is-danger")>
                                          <ybc::Title>
                                            {"Not Found"}
                                          </ybc::Title>
                                          {"No door with that name found."}
                                        </ybc::Notification>
                                        <a class={"button"} href={"/companies"} >{"Back"}</a>
                                      </>
                                    }

                                },
                              };
                              html!{
                                <>
                                  <ybc::Block>
                                    <ybc::Title>{"Name"}</ybc::Title>
                                    <input
                                      class=classes!("input")
                                      type={"text"}
                                      placeholder={"Name"}
                                      value={ company.name.to_owned() }
                                      onchange={self.link.callback(|event: ChangeData| {
                                        match event {
                                          ChangeData::Value(name) => Msg::Set(Action::Name, name),
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
                                      value={ company.description.to_owned() }
                                      onchange={self.link.callback(|event: ChangeData| {
                                        match event {
                                          ChangeData::Value(name) => Msg::Set(Action::Description, name),
                                          _ => Msg::Nothing,
                                        }
                                      }).clone()}
                                    />
                                  </ybc::Block>
                                  <hr/>
                                  { self.completion() }
                                  <div class="buttons">
                                    <ybc::Button
                                      classes=classes!("is-primary")
                                      onclick={
                                        match self.props.add {
                                          true => self.link.callback(|_| Msg::Add).clone(),
                                          false => self.link.callback(|_|  Msg::Update).clone(),
                                        }
                                      }
                                    >
                                      { "Save" }
                                    </ybc::Button>
                                    <ybc::Button
                                      classes=classes!("is-danger", "is-outlined")
                                      onclick={self.link.callback(|_| Msg::Delete).clone()}
                                    >
                                      { "Delete" }
                                    </ybc::Button>
                                  </div>
                                </>
                              }
                            },
                            None => html!{
                              <>
                                <ybc::Notification classes=classes!("is-info")>
                                  {"Fetching data ..."}
                                </ybc::Notification>
                                <a class={"button"} href={"/doors"} >{"Back"}</a>
                              </>
                            }
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

impl CompanyPage {
    fn completion(&self) -> Html {
        let mut value = 0.0;
        let company = match match &self.companies {
            Some(companies) => companies,
            None => panic!("No doors"),
        }
        .first()
        {
            Some(door) => door.clone(),
            None => panic!("No door"),
        };
        let opts = vec![company.name, company.description];
        for i in opts.clone().into_iter() {
            if i.len() > 0 {
                value += 1.0;
            }
        }
        html! {
          <ybc::Progress value=value max=opts.len() as f32 />
        }
    }
}
