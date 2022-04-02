use crate::pages::Page;
use yew::{function_component, html};
use yew_router::components::Link;

#[function_component(Sidemenu)]
pub fn sidemenu() -> Html {
    html! {
        <div class="wasmgl-sidemenu">
        <ul>
            <li><Link<Page> to={Page::HelloCanvas}>{ "HelloCanvas" }</Link<Page>></li>
            <li><Link<Page> to={Page::DrawingRectangle}>{ "DrawingRectangle" }</Link<Page>></li>
            <li><Link<Page> to={Page::HelloPoint1}>{ "HelloPoint1" }</Link<Page>></li>
            <li><Link<Page> to={Page::HelloPoint2}>{ "HelloPoint2" }</Link<Page>></li>
            <li><Link<Page> to={Page::ClickedPoints}>{ "ClickedPoints" }</Link<Page>></li>
            <li><Link<Page> to={Page::ColoredPoints}>{ "ColoredPoints" }</Link<Page>></li>

            <li><Link<Page> to={Page::MultiPoint}>{ "MultiPoint" }</Link<Page>></li>
            <li><Link<Page> to={Page::HelloTriangle}>{ "HelloTriangle" }</Link<Page>></li>
            <li><Link<Page> to={Page::HelloQuad}>{ "HelloQuad" }</Link<Page>></li>
            <li><Link<Page> to={Page::TranslatedTriangle}>{ "TranslatedTriangle" }</Link<Page>></li>
            <li><Link<Page> to={Page::TranslatedTriangle_Matrix}>{ "TranslatedTriangle_Matrix" }</Link<Page>></li>
            <li><Link<Page> to={Page::RotatedTriangle}>{ "RotatedTriangle" }</Link<Page>></li>
            <li><Link<Page> to={Page::RotatedTriangle_Matrix}>{ "RotatedTriangle_Matrix" }</Link<Page>></li>
            <li><Link<Page> to={Page::ScaledTriangle_Matrix}>{ "ScaledTriangle_Matrix" }</Link<Page>></li>

            <li><Link<Page> to={Page::RotatedTriangle_Matrix4}>{ "RotatedTriangle_Matrix4" }</Link<Page>></li>
            <li><Link<Page> to={Page::RotatedTranslatedTriangle}>{ "RotatedTranslatedTriangle" }</Link<Page>></li>
            <li><Link<Page> to={Page::RotatingTriangle}>{ "RotatingTriangle" }</Link<Page>></li>
            <li><Link<Page> to={Page::RotatingTranslatedTriangle}>{ "RotatingTranslatedTriangle" }</Link<Page>></li>

            <li><Link<Page> to={Page::MultiAttributeSize}>{ "MultiAttributeSize" }</Link<Page>></li>
            <li><Link<Page> to={Page::MultiAttributeSize_Interleaved}>{ "MultiAttributeSize_Interleaved" }</Link<Page>></li>
            <li><Link<Page> to={Page::ColoredTriangle}>{ "ColoredTriangle" }</Link<Page>></li>
            <li><Link<Page> to={Page::HelloTriangle_FragCoord}>{ "HelloTriangle_FragCoord" }</Link<Page>></li>
            <li><Link<Page> to={Page::TexturedQuad}>{ "TexturedQuad" }</Link<Page>></li>
            <li><Link<Page> to={Page::TexturedQuad_Repeat}>{ "TexturedQuad_Repeat" }</Link<Page>></li>
            <li><Link<Page> to={Page::TexturedQuad_Clamp_Mirror}>{ "TexturedQuad_Clamp_Mirror" }</Link<Page>></li>
            <li><Link<Page> to={Page::MultiTexture}>{ "MultiTexture" }</Link<Page>></li>

            <li><Link<Page> to={Page::LookAtTriangles}>{ "LookAtTriangles" }</Link<Page>></li>
            <li><Link<Page> to={Page::LookAtRotatedTriangles}>{ "LookAtRotatedTriangles" }</Link<Page>></li>
            <li><Link<Page> to={Page::LookAtTrianglesWithKeys}>{ "LookAtTrianglesWithKeys" }</Link<Page>></li>
        </ul>
        </div>
    }
}
