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
        </ul>
        </div>
    }
}
