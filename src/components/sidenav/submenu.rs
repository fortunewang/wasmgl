use yew::{Children, Properties};

#[derive(Debug, PartialEq, Properties)]
pub struct SubmenuProps {
    pub title: String,
    #[prop_or_default]
    pub children: Children,
}

#[yew::function_component(Submenu)]
pub fn submenu(props: &SubmenuProps) -> yew::Html {
    let expanded = yew::use_state_eq(|| false);
    let toggle = {
        let expanded = expanded.clone();
        yew::Callback::from(move |_| {
            expanded.set(!*expanded);
        })
    };
    yew::html! {
        <li class="wasmgl-sidenav-submenu">
            <div class="wasmgl-sidenav-submenu-title" onclick={toggle}>
                <span>{ props.title.clone() }</span>
                <i class={yew::classes!(
                    "wasmgl-sidenav-submenu-arrow",
                    if *expanded { None } else { Some("wasmgl-sidenav-submenu-arrow-collapsed") }
                )}></i>
            </div>
            if *expanded {
                <ul class="wasmgl-sidenav-submenu-items">
                    {props.children.clone()}
                </ul>
            }
        </li>
    }
}
