use yew::prelude::*;

#[derive(Clone, PartialEq, Eq, Properties)]
pub struct Props {
    #[prop_or_default]
    pub class: Classes,
}

#[function_component]
pub fn PageHeaderPlaceholder(props: &Props) -> Html {
    let Props { class } = props.clone();

    let class = classes!(
        "font-sans",
        "m-0",
        "rounded",
        "w-48",
        "h-4",
        "bg-washed-out-secondary",
        "animate-pulse",
        "cursor-progress",
        "mt-3",
        "mb-3",
        "lg:mb-7",
        class
    );

    html! {
        <h1 {class} aria-label="loading header"/>
    }
}
