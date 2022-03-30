mod ch02;
mod ch03;

use yew_router::{components::Redirect, Routable};

#[derive(Clone, PartialEq, Routable)]
pub enum Page {
    #[at("/")]
    Home,
    #[at("/ch02/hello_canvas")]
    HelloCanvas,
    #[at("/ch02/drawing_rectangle")]
    DrawingRectangle,
    #[at("/ch02/hello_point_1")]
    HelloPoint1,
    #[at("/ch02/hello_point_2")]
    HelloPoint2,
    #[at("/ch02/clicked_points")]
    ClickedPoints,
    #[at("/ch02/colored_points")]
    ColoredPoints,
    #[at("/ch03/multi_point")]
    MultiPoint,
    #[at("/ch03/hello_triangle")]
    HelloTriangle,
    #[at("/ch03/hello_quad")]
    HelloQuad,
}

pub fn render_page(page: &Page) -> yew::Html {
    use Page::*;
    match page {
        Home => yew::html! { <Redirect<Page> to={Page::HelloCanvas} />},
        HelloCanvas => yew::html! { <ch02::HelloCanvas /> },
        DrawingRectangle => yew::html! { <ch02::DrawingRectangle /> },
        HelloPoint1 => yew::html! { <ch02::HelloPoint1 /> },
        HelloPoint2 => yew::html! { <ch02::HelloPoint2 /> },
        ClickedPoints => yew::html! { <ch02::ClickedPoints /> },
        ColoredPoints => yew::html! { <ch02::ColoredPoints /> },

        MultiPoint => yew::html! { <ch03::MultiPoint /> },
        HelloTriangle => yew::html! { <ch03::HelloTriangle /> },
        HelloQuad => yew::html! { <ch03::HelloQuad /> },
    }
}
