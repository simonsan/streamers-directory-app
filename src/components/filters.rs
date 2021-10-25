use yew::prelude::*;
use yew_router::prelude::*;

use crate::routes::AppRoute;

/// Nav component
pub struct Filter;

impl Component for Filter {
    type Message = ();
    type Properties = ();

    fn create(_: Self::Properties,
              _: ComponentLink<Self>)
              -> Self {
        Filter {}
    }

    fn change(&mut self,
              _: Self::Properties)
              -> ShouldRender {
        false
    }

    fn update(&mut self,
              _: Self::Message)
              -> ShouldRender {
        true
    }

    fn view(&self) -> Html {
        html! {
            <section class="section">
                <nav class="panel">
              <p class="panel-heading">
                { "Content" }
              </p>
            // TODO: Use search box?
            //   <div class="panel-block">
            //     <p class="control has-icons-left">
            //       <input class="input" type="text" placeholder="Search" />
            //       <span class="icon is-left">
            //         <i class="fas fa-search" aria-hidden="true"></i>
            //       </span>
            //     </p>
            //   </div>
              <p class="panel-tabs">
                <a class="is-active">{ "All" }</a>
                <a>{ "AoE I" }</a>
                <a>{ "AoE II" }</a>
                <a>{ "AoE III" }</a>
                <a>{ "AoE IV" }</a>
              </p>
              <label class="panel-block">
                <input type="checkbox" />
                { "Casting" }
              </label>
              <label class="panel-block">
                <input type="checkbox" />
                { "POV" }
              </label>
              <label class="panel-block">
                <input type="checkbox" />
                { "Community Games" }
              </label>
              <div class="panel-block">
                <button class="button is-link is-outlined is-fullwidth">
                  { "Reset all filters" }
                </button>
              </div>
            </nav>
            </section>
        }
    }
}
