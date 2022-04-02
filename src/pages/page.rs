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
    #[at("/ch05/hello_triangle_frag_coord")]
    HelloTriangle_FragCoord,
    #[at("/ch05/textured_quad")]
    TexturedQuad,
    #[at("/ch05/textured_quad_repeat")]
    TexturedQuad_Repeat,
    #[at("/ch05/textured_quad_clamp_mirror")]
    TexturedQuad_Clamp_Mirror,
    #[at("/ch05/multi_texture")]
    MultiTexture,
    #[at("/ch07/look_at_triangles")]
    LookAtTriangles,
    #[at("/ch07/look_at_rotated_triangles")]
    LookAtRotatedTriangles,
    #[at("/ch07/look_at_triangles_with_keys")]
    LookAtTrianglesWithKeys,
}

pub fn render_page(page: &Page) -> yew::Html {
    use Page::*;
    match page {
        Home => yew::html! { <Redirect<Page> to={Page::HelloCanvas} />},

        HelloCanvas => yew::html! { <super::ch02::HelloCanvas /> },
        DrawingRectangle => yew::html! { <super::ch02::DrawingRectangle /> },
        HelloPoint1 => yew::html! { <super::ch02::HelloPoint1 /> },
        HelloPoint2 => yew::html! { <super::ch02::HelloPoint2 /> },
        ClickedPoints => yew::html! { <super::ch02::ClickedPoints /> },
        ColoredPoints => yew::html! { <super::ch02::ColoredPoints /> },

        MultiPoint => yew::html! { <super::ch03::MultiPoint /> },
        HelloTriangle => yew::html! { <super::ch03::HelloTriangle /> },
        HelloQuad => yew::html! { <super::ch03::HelloQuad /> },
        TranslatedTriangle => yew::html! { <super::ch03::TranslatedTriangle /> },
        TranslatedTriangle_Matrix => yew::html! { <super::ch03::TranslatedTriangle_Matrix /> },
        RotatedTriangle => yew::html! { <super::ch03::RotatedTriangle /> },
        RotatedTriangle_Matrix => yew::html! { <super::ch03::RotatedTriangle_Matrix /> },
        ScaledTriangle_Matrix => yew::html! { <super::ch03::ScaledTriangle_Matrix /> },

        RotatedTriangle_Matrix4 => yew::html! { <super::ch04::RotatedTriangle_Matrix4 /> },
        RotatedTranslatedTriangle => yew::html! { <super::ch04::RotatedTranslatedTriangle /> },
        RotatingTriangle => yew::html! { <super::ch04::RotatingTriangle /> },
        RotatingTranslatedTriangle => yew::html! { <super::ch04::RotatingTranslatedTriangle /> },

        MultiAttributeSize => yew::html! { <super::ch05::MultiAttributeSize /> },
        MultiAttributeSize_Interleaved => {
            yew::html! { <super::ch05::MultiAttributeSize_Interleaved /> }
        }
        ColoredTriangle => yew::html! { <super::ch05::ColoredTriangle /> },
        HelloTriangle_FragCoord => yew::html! { <super::ch05::HelloTriangle_FragCoord /> },
        TexturedQuad => yew::html! { <super::ch05::TexturedQuad /> },
        TexturedQuad_Repeat => yew::html! { <super::ch05::TexturedQuad_Repeat /> },
        TexturedQuad_Clamp_Mirror => yew::html! { <super::ch05::TexturedQuad_Clamp_Mirror /> },
        MultiTexture => yew::html! { <super::ch05::MultiTexture /> },

        LookAtTriangles => yew::html! { <super::ch07::LookAtTriangles /> },
        LookAtRotatedTriangles => yew::html! { <super::ch07::LookAtRotatedTriangles /> },
        LookAtTrianglesWithKeys => yew::html! { <super::ch07::LookAtTrianglesWithKeys /> },
    }
}
