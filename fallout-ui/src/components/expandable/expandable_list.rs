use yew::prelude::*;
use yew_heroicons::size_20::solid::ChevronDownIcon;
use yew_heroicons::size_20::solid::ChevronRightIcon;

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    pub expanded: bool,
    pub toggle_expanded: Callback<()>,
    #[prop_or(String::from("Toggle me"))]
    pub button_text: String,
    #[prop_or_default]
    pub class: Classes,
    #[prop_or_default]
    pub button_class: Classes,
    #[prop_or_default]
    pub icon_class: String,
    #[prop_or_default]
    pub list_class: Classes,
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub data_qa: String,
}

#[function_component]
pub fn ExpandableList(props: &Props) -> Html {
    let Props {
        expanded,
        toggle_expanded,
        button_text,
        children,
        class,
        button_class,
        mut icon_class,
        list_class,
        data_qa,
    } = props.clone();

    let data_qa = format!("{data_qa}-expandable-list");

    let onclick = toggle_expanded.reform(|_| ());

    icon_class += " w-6";

    html! {
        <div class={class} data-qa={data_qa.clone()}>
            <button
                class={classes!(
                    "border-none", "bg-transparent", "flex", "justify-between", "w-full", "pl-0", "pr-0", "pb-0.5", "pt-0", "hover:underline", "cursor-pointer",
                    "font-sans", "m-0", "text-secondary", "text-base",
                    button_class
                )}
                {onclick}
                {data_qa}
            >
                {button_text}
                if expanded {
                    <ChevronDownIcon class={icon_class}/>
                } else {
                    <ChevronRightIcon class={icon_class}/>
                }
            </button>
            <ul class={classes!("list-none", "m-0", "pl-4","pr-0","pb-0","pt-0","flex","flex-col","gap-0.5", list_class)}>
                {children}
            </ul>
        </div>
    }
}
