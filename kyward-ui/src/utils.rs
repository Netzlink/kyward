use yew::prelude::*;

pub fn new_hero(title: &'static str, subtitle: &'static str) -> Html {
    html! {
      <section class=classes!("hero", "is-primary", "is-small")>
        <div class=classes!("hero-body")>
          <p class=classes!("title")>
            {title}
          </p>
          <p class=classes!("subtitle")>
            {subtitle}
          </p>
        </div>
      </section>
    }
}
