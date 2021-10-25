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
                                            { "Games" }
                                        </a>
                                            <div class="navbar-dropdown">
                                                <a class="navbar-item">
                                                    { "AoE2" }
                                                </a>
                                                <a class="navbar-item">
                                                    { "AoE4" }
                                                </a>
                                            </div>
                                    </div>
                                    <div class="navbar-item has-dropdown is-hoverable">
                                        <a class="navbar-link">
                                            { "Category" }
                                        </a>
                                            <div class="navbar-dropdown">
                                                <a class="navbar-item">
                                                    { "Casting" }
                                                </a>
                                                <a class="navbar-item">
                                                    { "POV" }
                                                </a>
                                                <a class="navbar-item">
                                                    { "Community Games" }
                                                </a>
                                            </div>
                                    </div>
                            </div>
                </div>
            </nav>
        }
    }
}
