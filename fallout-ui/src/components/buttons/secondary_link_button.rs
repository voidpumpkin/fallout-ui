use yew::prelude::*;

use crate::components::text_spinner::TextSpinner;
use crate::hooks::use_buttons_shared_logic;

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    pub onclick: Callback<MouseEvent>,
    #[prop_or_default]
    pub loading: bool,
    #[prop_or_default]
    pub disabled: bool,

    // Use this for displaying icons. When the component is set as loading, this component will be
    // a loading icon instead.
    #[prop_or_default]
    pub decoration_left: Option<Html>,

    #[prop_or_default]
    pub title: Option<String>,
    #[prop_or_default]
    pub class: Classes,
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub data_qa: String,
}

#[function_component]
pub fn SecondaryLinkButton(props: &Props) -> Html {
    let Props {
        class,
        loading,
        disabled,
        mut decoration_left,
        children,
        onclick,
        data_qa,
        title,
    } = props.clone();

    let onclick = use_buttons_shared_logic(onclick, disabled, loading);

    let class = classes!(
        "m-0",
        "border-none",
        "bg-transparent",
        "select-text",
        "font-sans",
        "text-base",
        "text-secondary",
        "hover:underline",
        "hover:text-secondary",
        "cursor-pointer",
        class
    );

    if loading {
        decoration_left = decoration_left.map(|_| {
            html! {
                <TextSpinner />
            }
        });
    }

    html! {
        <button {onclick} {class} {disabled} data-qa={format!("{data_qa}-secondary-link-button")} {title}>
            if let Some(decoration_left) = decoration_left {
                <div class="min-w-5 w-5 mr-2 pb-0.5 inline-block">
                    {decoration_left}
                </div>
            }
            {children}
        </button>
    }
}
