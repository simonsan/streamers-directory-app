use yew::prelude::*;

use crate::components::filters::Filter;

/// Home page
pub struct Home;

impl Component for Home {
    type Message = ();
    type Properties = ();

    fn create(_: Self::Properties,
              _: ComponentLink<Self>)
              -> Self {
        Home {}
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
        <>
        <div class="columns">
            <div class={ "column" }>
            <Filter />
            </div>
            <div class={ "column is-two-thirds" }>
            { "Placeholder: StreamerList" }
            </div>
        </div>
        </>

              }
    }
}
