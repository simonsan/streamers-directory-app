use yew::prelude::*;
use yew_router::prelude::*;

use crate::routes::AppRoute;

/// Nav component
pub struct Nav;

impl Component for Nav {
    type Message = ();
    type Properties = ();

    fn create(_: Self::Properties,
              _: ComponentLink<Self>)
              -> Self {
        Nav {}
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
        // TODO: https://yew.rs/concepts/html/lists
        // Create list of menu items from enums

        html! {
            <nav class="navbar" role="navigation" aria-label="main navigation">

                <div class="navbar-menu">
                            <div class="navbar-start">
                            <RouterAnchor<AppRoute> route=AppRoute::Home classes="app-link navbar-item" >{ "Home" }</RouterAnchor<AppRoute>>
                            <RouterAnchor<AppRoute> route=AppRoute::About classes="app-link navbar-item">{ "About" }</RouterAnchor<AppRoute>>
                            <hr class="navbar-divider" />
                                    <div class="navbar-item has-dropdown is-hoverable">
                                        <a class="navbar-link">
                                            { "Languages" }
                                        </a>
                                            <div class="navbar-dropdown">
                                                <a class="navbar-item">
                                                    { "En" }
                                                </a>
                                                <a class="navbar-item">
                                                    { "De" }
                                                </a>
                                            </div>
                                    </div>
                            </div>
                </div>
            </nav>
        }
    }
}
