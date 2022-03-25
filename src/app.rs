use crate::layout::Layout;
use crate::pages::{render_page, Page};
use yew::function_component;
use yew_router::{BrowserRouter, Switch};

#[function_component(App)]
pub fn app() -> yew::Html {
    yew::html! {
        <BrowserRouter>
        <Layout>
            <Switch<Page> render={Switch::render(render_page)} />
        </Layout>
        </BrowserRouter>
    }
}
