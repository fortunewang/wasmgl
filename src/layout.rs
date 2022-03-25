use crate::sidemenu::Sidemenu;
use yew::{function_component, html, Children, Properties};

#[derive(Debug, PartialEq, Properties)]
pub struct LayoutProps {
    #[prop_or_default]
    pub children: Children,
}

#[function_component(Layout)]
pub fn layout(props: &LayoutProps) -> Html {
    html! {
        <div class="wasmgl-layout">
            <Sidemenu />
            <div class="wasmgl-content">
                {props.children.clone()}
            </div>
        </div>
    }
}
