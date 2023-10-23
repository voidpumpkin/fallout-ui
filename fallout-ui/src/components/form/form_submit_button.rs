use yew::prelude::*;

use crate::components::text_spinner::TextSpinner;
use crate::hooks::use_buttons_shared_logic;

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    #[prop_or(String::from("Submit"))]
    pub text: String,
    #[prop_or_default]
    pub disabled: bool,
    #[prop_or_default]
    pub loading: bool,
    #[prop_or_default]
    pub title: Option<String>,
    #[prop_or_default]
    pub onclick: Callback<MouseEvent>,
    #[prop_or_default]
    pub data_qa: String,
}

#[function_component]
pub fn FormSubmitButton(props: &Props) -> Html {
    let Props {
        text,
        disabled,
        loading,
        onclick,
        data_qa,
        title,
    } = props.clone();

    let onclick = use_buttons_shared_logic(onclick, disabled, loading);

    let class = classes!(
        "border-none",
        "inline-flex",
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
        "inline-flex",
        "mb-3",
        "mr-3",
        "max-w-fit",
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
    );

    html! {
        <button
            data-qa={format!("{data_qa}-form-submit-button")}
            type={"submit"}
            {class}
            {disabled}
            {onclick}
            {title}
        >
            if loading {
                <TextSpinner />
            }
            {text}
        </button>
    }
}
