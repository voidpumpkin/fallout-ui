use yew::prelude::*;

use crate::components::link::hooks::use_primary_link_logic::use_primary_link_logic;

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    pub href: String,
    #[prop_or_default]
    pub class: Classes,
    #[prop_or_default]
    pub children: Children,
}

#[function_component]
pub fn PrimaryAnchor(props: &Props) -> Html {
    let Props {
        class,
        children,
        href,
    } = props.clone();

    let class = use_primary_link_logic(class);

    html! {
        <a {href} {class}>
            {children}
        </a>
    }
}
