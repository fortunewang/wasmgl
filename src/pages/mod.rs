mod ch02;
mod ch03;
mod ch04;
mod ch05;

use yew_router::{components::Redirect, Routable};

#[allow(non_camel_case_types)]
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
    #[at("/ch03/translated_triangle")]
    TranslatedTriangle,
    #[at("/ch03/translated_triangle_matrix")]
    TranslatedTriangle_Matrix,
    #[at("/ch03/rotated_triangle")]
    RotatedTriangle,
    #[at("/ch03/rotated_triangle_matrix")]
    RotatedTriangle_Matrix,
    #[at("/ch03/scaled_triangle_matrix")]
    ScaledTriangle_Matrix,
    #[at("/ch04/rotated_triangle_matrix4")]
    RotatedTriangle_Matrix4,
    #[at("/ch04/rotated_translated_triangle")]
    RotatedTranslatedTriangle,
    #[at("/ch04/rotating_triangle")]
    RotatingTriangle,
    #[at("/ch04/rotating_translated_triangle")]
    RotatingTranslatedTriangle,
    #[at("/ch05/multi_attribute_size")]
    MultiAttributeSize,
    #[at("/ch05/multi_attribute_size_interleaved")]
    MultiAttributeSize_Interleaved,
    #[at("/ch05/colored_triangle")]
    ColoredTriangle,
    #[at("/ch05/textured_quad")]
    TexturedQuad,
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
        TranslatedTriangle => yew::html! { <ch03::TranslatedTriangle /> },
        TranslatedTriangle_Matrix => yew::html! { <ch03::TranslatedTriangle_Matrix /> },
        RotatedTriangle => yew::html! { <ch03::RotatedTriangle /> },
        RotatedTriangle_Matrix => yew::html! { <ch03::RotatedTriangle_Matrix /> },
        ScaledTriangle_Matrix => yew::html! { <ch03::ScaledTriangle_Matrix /> },

        RotatedTriangle_Matrix4 => yew::html! { <ch04::RotatedTriangle_Matrix4 /> },
        RotatedTranslatedTriangle => yew::html! { <ch04::RotatedTranslatedTriangle /> },
        RotatingTriangle => yew::html! { <ch04::RotatingTriangle /> },
        RotatingTranslatedTriangle => yew::html! { <ch04::RotatingTranslatedTriangle /> },

        MultiAttributeSize => yew::html! { <ch05::MultiAttributeSize /> },
        MultiAttributeSize_Interleaved => yew::html! { <ch05::MultiAttributeSize_Interleaved /> },
        ColoredTriangle => yew::html! { <ch05::ColoredTriangle /> },
        TexturedQuad => yew::html! { <ch05::TexturedQuad /> },
    }
}
