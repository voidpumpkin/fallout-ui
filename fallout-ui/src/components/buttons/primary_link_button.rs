use yew::prelude::*;

use crate::hooks::use_buttons_shared_logic;

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    pub onclick: Callback<MouseEvent>,
    #[prop_or_default]
    pub title: Option<String>,
    #[prop_or_default]
    pub class: Classes,
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub data_qa: String,
    #[prop_or_default]
    pub disabled: bool,
}

#[function_component]
pub fn PrimaryLinkButton(props: &Props) -> Html {
    let Props {
        class,
        children,
        onclick,
        data_qa,
        title,
        disabled,
    } = props.clone();

    let onclick = use_buttons_shared_logic(onclick, false, false);

    let class = classes!(
        "m-0",
        "border-none",
        "bg-transparent",
        "select-text",
        "font-sans",
        "text-base",
        "text-primary",
        "hover:underline",
        "hover:text-primary",
        "cursor-pointer",
        class
    );

    html! {
        <button {disabled} {onclick} {class} data-qa={format!("{data_qa}-primary-link-button")} {title}>
            {children}
        </button>
    }
}
