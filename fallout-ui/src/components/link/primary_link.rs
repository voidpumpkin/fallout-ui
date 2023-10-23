use yew::prelude::*;
use yew_router::prelude::Link as YewLink;
use yew_router::Routable;

use crate::components::link::hooks::use_data_qa_manual_edit::use_data_qa_manual_edit;
use crate::components::link::hooks::use_primary_link_logic::use_primary_link_logic;
use crate::utils::to_data_qa;

#[derive(Clone, PartialEq, Properties)]
pub struct Props<T: Routable + 'static> {
    pub to: T,
    #[prop_or_default]
    pub class: Classes,
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub data_qa: String,
}

#[function_component]
pub fn PrimaryLink<T: Routable + 'static>(props: &Props<T>) -> Html {
    let Props {
        class,
        children,
        to,
        data_qa,
    } = props.clone();

    let class = use_primary_link_logic(class);

    let data_qa = to_data_qa(format!("{data_qa}-primary-link"));
    let anchor_ref = use_data_qa_manual_edit(data_qa);

    html! {
        <YewLink<T> {to} classes={class} {anchor_ref}>
            {children}
        </YewLink<T>>
    }
}
