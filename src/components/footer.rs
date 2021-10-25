use yew::{
    html,
    Component,
    ComponentLink,
    Html,
    ShouldRender,
};


pub struct Footer {}

pub enum Msg {}

impl Component for Footer {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties,
              _: ComponentLink<Self>)
              -> Self {
        Footer {}
    }

    fn update(&mut self,
              _msg: Self::Message)
              -> ShouldRender {
        true
    }

    fn change(&mut self,
              _: Self::Properties)
              -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <footer class="footer">
                <div class="content has-text-centered">
                <p>
                    <span class="attribution">
                        { "© 2021. A directory list of content creators made by " }
                        <a href="https://github.com/simonsan/streamers-directory-app"> { "Simonsan" } </a>
                        { ". Code licensed under AGPLv3" }
                    </span>
                </p>
            </div>
            </footer>
        }
    }
}
