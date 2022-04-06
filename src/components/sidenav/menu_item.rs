use yew::{Children, Properties};

#[derive(Debug, PartialEq, Properties)]
pub struct MenuItemProps {
    #[prop_or_default]
    pub children: Children,
}

#[yew::function_component(MenuItem)]
pub fn menu_item(props: &MenuItemProps) -> yew::Html {
    yew::html! {
        <li class="wasmgl-sidenav-menu-item">
            {props.children.clone()}
        </li>
    }
}
