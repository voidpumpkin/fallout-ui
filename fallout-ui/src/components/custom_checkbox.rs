use yew::prelude::*;
use yew_heroicons::size_20::solid::CheckIcon;

#[derive(Clone, PartialEq, Eq, Properties)]
pub struct Props {
    #[prop_or_default]
    pub checked: bool,
}

#[function_component]
pub fn CustomCheckbox(props: &Props) -> Html {
    let Props { checked } = props.clone();

    let class = classes!(
        "flex",
        "box-border",
        "w-4",
        "h-4",
        "grow-0",
        "shrink-0",
        if checked { "bg-primary" } else { "bg-white" },
        "text-white",
        if checked {
            "border-primary"
        } else {
            "border-secondary"
        },
        "border-solid",
        "border-[2px]",
        "rounded-sm",
    );

    html! {
        <div {class}>
            <CheckIcon />
        </div>
    }
}
