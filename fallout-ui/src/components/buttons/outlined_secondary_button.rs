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

    // Use this for displaying icons. When the component is set as loading, this component will be
    // a loading icon instead.
    #[prop_or_default]
    pub decoration_left: Option<Html>,

    #[prop_or_default]
    pub class: Classes,
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub data_qa: String,
}

#[function_component]
pub fn OutlinedSecondaryButton(props: &Props) -> Html {
    let Props {
        class,
        children,
        onclick,
        mut decoration_left,
        data_qa,
        loading,
        disabled,
        title,
    } = props.clone();

    let onclick = use_buttons_shared_logic(onclick, disabled, loading);

    let class = classes!(
        "border-1",
        "border-solid",
        "flex",
        "justify-center",
        "items-center",
        "gap-2",
        "py-2",
        "font-sans",
        "font-normal",
        "text-base",
        "text-center",
        "rounded-full",
        if decoration_left.is_some() {
            classes!(
                "pl-5",
                "pr-10",
                "border-secondary",
                "text-secondary",
                "bg-transparent",
                "hover:bg-secondary",
                "hover:text-white",
                "cursor-pointer"
            )
        } else if disabled {
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
        <button {onclick} {class} {disabled} data-qa={format!("{data_qa}-outlined-secondary-button")} {title}>
                if let Some(decoration_left) = decoration_left {
                    <div class="min-w-5 w-5 mr-2 pb-0.5 inline-block">
                        {decoration_left}
                    </div>
                } else if loading {
                    <TextSpinner />
                }
            {children}
        </button>
    }
}
