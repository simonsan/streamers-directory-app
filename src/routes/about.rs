use yew::prelude::*;

/// About page
pub struct About;

impl Component for About {
    type Message = ();
    type Properties = ();

    fn create(_: Self::Properties,
              _: ComponentLink<Self>)
              -> Self {
        About {}
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

                    <section class="hero">
          <div class="hero-body">
            <p class="title">
              { "AoE Streamers Directory" }
            </p>
            <p class="subtitle">
              { " A directory of content creators in the Age of Empires universe" }
            </p>
          </div>
        </section>

                }
    }
}

#[cfg(test)]
mod tests {
    use wasm_bindgen_test::*;

    wasm_bindgen_test_configure!(run_in_browser);

    use yew::App;

    use super::About;

    #[wasm_bindgen_test]
    fn about_page_has_an_app_link() {
        let app: App<About> = yew::App::new();
        app.mount(yew::utils::document().get_element_by_id("output").unwrap());

        let app_links =
            yew::utils::document().get_elements_by_class_name("app-link");

        assert_eq!(app_links.length(), 1);

        let link = app_links.item(0).expect("No app-link").inner_html();
        assert_eq!(link, "Create Yew App");
    }
}
