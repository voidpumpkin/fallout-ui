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
    #[prop_or_default]
    pub title: Option<String>,
    #[prop_or_default]
    pub class: Classes,
    #[prop_or(String::from("button"))]
    pub button_type: String,
    #[prop_or_default]
    pub form: String,
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub data_qa: String,
}

#[function_component]
pub fn PrimaryButton(props: &Props) -> Html {
    let Props {
        class,
        mut button_type,
        form,
        children,
        onclick,
        data_qa,
        loading,
        disabled,
        title,
    } = props.clone();

    let onclick = use_buttons_shared_logic(onclick, disabled, loading);

    let class = classes!(
        "border-none",
        "flex",
        "justify-center",
        "items-center",
        "gap-2",
        "py-2",
        "font-sans",
        "font-normal",
        "text-base",
        "text-white",
        "text-center",
        "rounded-full",
        if disabled {
            classes!("px-10", "bg-washed-out-primary", "cursor-not-allowed")
        } else if loading {
            classes!("px-7", "bg-washed-out-primary", "cursor-progress")
        } else {
            classes!(
                "px-10",
                "bg-primary",
                "hover:bg-reacting-primary",
                "cursor-pointer"
            )
        },
        class
    );

    if !form.is_empty() {
        button_type = String::from("submit");
    }

    html! {
        <button
            {onclick}
            data-qa={format!("{data_qa}-primary-button")}
            type={button_type}
            {form}
            {title}
            {class}
            {disabled}
        >
            if loading {
                <TextSpinner />
            }
            {children}
        </button>
    }
}
