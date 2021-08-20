use super::super::models::company::Company;
use super::super::utils::new_hero;
use ybc::TileCtx::{Child, Parent};
use yew::format::{Json, Nothing};
use yew::prelude::*;
use yew::services::fetch::FetchService;
use yew::services::fetch::{FetchTask, Request, Response};
use yew::services::ConsoleService;
use yew::web_sys;

pub enum Msg {
  GetResp(Result<Vec<Company>, anyhow::Error>),
  Refresh,
  Add,
}

#[derive(Clone, Properties, PartialEq)]
pub struct Properties {
  pub token: String,
}

pub struct Companies {
  link: ComponentLink<Self>,
  companies: Vec<Company>,
  fetching: Option<FetchTask>,
  props: Properties,
}

impl Component for Companies {
  type Message = Msg;
  type Properties = Properties;

  fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
    return |companies: Companies| -> Companies {
      companies
        .link
        .callback(|_: Msg| Msg::Refresh)
        .emit(Msg::Refresh);
      companies
    }(Self {
      link,
      fetching: None,
      companies: vec![],
      props: props,
    });
  }

  fn update(&mut self, msg: Self::Message) -> ShouldRender {
    match msg {
      Msg::GetResp(resp) => {
        self.companies = resp.expect("Test");
        true
      }
      Msg::Refresh => {
        let req = Request::get("/api/v1alpha1/company")
          .header("Authorization", format!("Bearer {0}", self.props.token))
          .body(Nothing)
          .expect("can make req to jsonplaceholder");

        let cb = self.link.callback(
          |response: Response<Json<Result<Vec<Company>, anyhow::Error>>>| {
            let Json(data) = response.into_body();
            Msg::GetResp(data)
          },
        );

        let task = FetchService::fetch(req, cb).expect("can create task");
        self.fetching = Some(task);
        true
      }
      Msg::Add => {
        let next = match self.companies.last() {
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
          .set_pathname(format!("/company/add/{0}", next).as_str())
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
        {new_hero("Companies", "Manage companies accessible by a person.")}
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
                        <th>{"Description"}</th>
                      </tr>
                    </thead>
                    <tbody>
                    {
                      self.companies.iter().map( |company|
                        html!{
                          <tr>
                            <th><a href={format!("/company/{0}", &company.id)}>{ &company.name }</a></th>
                            <th>{ &company.description }</th>
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
