use yew::prelude::*;

use crate::components::link::hooks::use_secondary_link_logic::use_secondary_link_logic;

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    pub href: String,
    #[prop_or_default]
    pub text_classes: Option<Callback<bool, Classes>>,
    #[prop_or_default]
    pub class: Classes,
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub has_error: bool,
}

#[function_component]
pub fn SecondaryAnchor(props: &Props) -> Html {
    let Props {
        class,
        text_classes,
        children,
        href,
        has_error,
    } = props.clone();

    let class = use_secondary_link_logic(class, has_error, text_classes);

    html! {
        <a {href} {class}>
            {children}
        </a>
    }
}
