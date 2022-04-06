use yew::{Children, Properties};

#[derive(Debug, PartialEq, Properties)]
pub struct MenuProps {
    #[prop_or_default]
    pub children: Children,
}

#[yew::function_component(Menu)]
pub fn menu(props: &MenuProps) -> yew::Html {
    yew::html! {
        <ul class="wasmgl-sidenav-menu">
            {props.children.clone()}
        </ul>
    }
}
