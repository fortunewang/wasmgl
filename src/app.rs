use crate::pages::{render_page, Page};
use crate::sidemenu::Sidemenu;
use yew::function_component;
use yew_router::{BrowserRouter, Switch};

#[function_component(App)]
pub fn app() -> yew::Html {
    yew::html! {
        <BrowserRouter>
        <div class="wasmgl-layout">
            <Sidemenu />
            <div class="wasmgl-content">
                <Switch<Page> render={render_page} />
            </div>
        </div>
        </BrowserRouter>
    }
}
