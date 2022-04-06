use crate::components::sidenav::{Menu, MenuItem, Submenu};
use crate::pages::Page;
use yew::{function_component, html};
use yew_router::components::Link;

#[function_component(Sidemenu)]
pub fn sidemenu() -> Html {
    html! {
        <div class="wasmgl-sidemenu">
        <Menu>
            <Submenu title="ch02">
                <MenuItem><Link<Page> to={Page::HelloCanvas}>{ "HelloCanvas" }</Link<Page>></MenuItem>
                <MenuItem><Link<Page> to={Page::DrawingRectangle}>{ "DrawingRectangle" }</Link<Page>></MenuItem>
                <MenuItem><Link<Page> to={Page::HelloPoint1}>{ "HelloPoint1" }</Link<Page>></MenuItem>
                <MenuItem><Link<Page> to={Page::HelloPoint2}>{ "HelloPoint2" }</Link<Page>></MenuItem>
                <MenuItem><Link<Page> to={Page::ClickedPoints}>{ "ClickedPoints" }</Link<Page>></MenuItem>
                <MenuItem><Link<Page> to={Page::ColoredPoints}>{ "ColoredPoints" }</Link<Page>></MenuItem>
            </Submenu>
            <Submenu title="ch03">
                <MenuItem><Link<Page> to={Page::MultiPoint}>{ "MultiPoint" }</Link<Page>></MenuItem>
                <MenuItem><Link<Page> to={Page::HelloTriangle}>{ "HelloTriangle" }</Link<Page>></MenuItem>
                <MenuItem><Link<Page> to={Page::HelloQuad}>{ "HelloQuad" }</Link<Page>></MenuItem>
                <MenuItem><Link<Page> to={Page::TranslatedTriangle}>{ "TranslatedTriangle" }</Link<Page>></MenuItem>
                <MenuItem><Link<Page> to={Page::TranslatedTriangle_Matrix}>{ "TranslatedTriangle_Matrix" }</Link<Page>></MenuItem>
                <MenuItem><Link<Page> to={Page::RotatedTriangle}>{ "RotatedTriangle" }</Link<Page>></MenuItem>
                <MenuItem><Link<Page> to={Page::RotatedTriangle_Matrix}>{ "RotatedTriangle_Matrix" }</Link<Page>></MenuItem>
                <MenuItem><Link<Page> to={Page::ScaledTriangle_Matrix}>{ "ScaledTriangle_Matrix" }</Link<Page>></MenuItem>
            </Submenu>
            <Submenu title="ch04">
                <MenuItem><Link<Page> to={Page::RotatedTriangle_Matrix4}>{ "RotatedTriangle_Matrix4" }</Link<Page>></MenuItem>
                <MenuItem><Link<Page> to={Page::RotatedTranslatedTriangle}>{ "RotatedTranslatedTriangle" }</Link<Page>></MenuItem>
                <MenuItem><Link<Page> to={Page::RotatingTriangle}>{ "RotatingTriangle" }</Link<Page>></MenuItem>
                <MenuItem><Link<Page> to={Page::RotatingTranslatedTriangle}>{ "RotatingTranslatedTriangle" }</Link<Page>></MenuItem>
            </Submenu>
            <Submenu title="ch05">
                <MenuItem><Link<Page> to={Page::MultiAttributeSize}>{ "MultiAttributeSize" }</Link<Page>></MenuItem>
                <MenuItem><Link<Page> to={Page::MultiAttributeSize_Interleaved}>{ "MultiAttributeSize_Interleaved" }</Link<Page>></MenuItem>
                <MenuItem><Link<Page> to={Page::ColoredTriangle}>{ "ColoredTriangle" }</Link<Page>></MenuItem>
                <MenuItem><Link<Page> to={Page::HelloTriangle_FragCoord}>{ "HelloTriangle_FragCoord" }</Link<Page>></MenuItem>
                <MenuItem><Link<Page> to={Page::TexturedQuad}>{ "TexturedQuad" }</Link<Page>></MenuItem>
                <MenuItem><Link<Page> to={Page::TexturedQuad_Repeat}>{ "TexturedQuad_Repeat" }</Link<Page>></MenuItem>
                <MenuItem><Link<Page> to={Page::TexturedQuad_Clamp_Mirror}>{ "TexturedQuad_Clamp_Mirror" }</Link<Page>></MenuItem>
                <MenuItem><Link<Page> to={Page::MultiTexture}>{ "MultiTexture" }</Link<Page>></MenuItem>
            </Submenu>
            <Submenu title="ch07">
                <MenuItem><Link<Page> to={Page::LookAtTriangles}>{ "LookAtTriangles" }</Link<Page>></MenuItem>
                <MenuItem><Link<Page> to={Page::LookAtRotatedTriangles}>{ "LookAtRotatedTriangles" }</Link<Page>></MenuItem>
                <MenuItem><Link<Page> to={Page::LookAtTrianglesWithKeys}>{ "LookAtTrianglesWithKeys" }</Link<Page>></MenuItem>
            </Submenu>
        </Menu>
        </div>
    }
}
