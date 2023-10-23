use yew::prelude::*;

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    #[prop_or(true)]
    pub flash_orange_on_click: bool,
    #[prop_or_default]
    pub disabled: bool,
    #[prop_or_default]
    pub title: Option<String>,
    #[prop_or_default]
    pub onclick: Callback<MouseEvent>,
    #[prop_or_default]
    pub children: Children,
}

#[function_component]
pub fn IconButtonForInputDecoration(props: &Props) -> Html {
    let Props {
        disabled,
        onclick,
        children,
        flash_orange_on_click,
        title,
    } = props.clone();

    let button_class = classes!(
        "p-0",
        "border-0",
        "bg-transparent",
        "text-base",
        "w-5",
        "h-5",
        "cursor-pointer",
        "flex",
        "grow-0",
        "shrink-0",
        if disabled {
            classes!("text-washed-out-secondary", "cursor-not-allowed")
        } else {
            classes!(
                "text-secondary",
                "hover:text-reacting-secondary",
                flash_orange_on_click.then_some("active:text-reacting-primary"),
            )
        }
    );

    html! {
        <button class={button_class} type="button" {onclick} tabindex="-1" {disabled} {title}>
            {children}
        </button>
    }
}
