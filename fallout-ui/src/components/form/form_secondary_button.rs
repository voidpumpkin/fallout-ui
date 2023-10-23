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
pub fn FormSecondaryButton(props: &Props) -> Html {
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
        "border-1",
        "border-solid",
        "inline-flex",
        "justify-center",
        "items-center",
        "gap-2",
        "py-[calc(0.5rem-1px)]",
        "font-sans",
        "font-normal",
        "text-base",
        "text-center",
        "rounded-full",
        "max-w-fit",
        "mb-3",
        if disabled {
            classes!(
                "px-10",
                "border-washed-out-secondary",
                "text-washed-out-secondary",
                "bg-transparent",
                "cursor-not-allowed"
            )
        } else if loading {
            classes!(
                "px-7",
                "border-washed-out-secondary",
                "text-washed-out-secondary",
                "bg-transparent",
                "cursor-progress"
            )
        } else {
            classes!(
                "border-secondary",
                "px-10",
                "text-secondary",
                "bg-transparent",
                "hover:bg-secondary",
                "hover:text-white",
                "cursor-pointer"
            )
        },
    );

    html! {
        <button
            data-qa={format!("{data_qa}-form-secondary-button")}
            type={"button"}
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
