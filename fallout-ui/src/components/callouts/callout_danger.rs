use yew::prelude::*;
use yew_heroicons::size_24::outline::ExclamationCircleIcon;

use crate::components::typography::body_text::BodyText;

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    pub children: Children,
    #[prop_or_default]
    pub class: Classes,
}

#[function_component]
pub fn CalloutDanger(props: &Props) -> Html {
    let Props { children, class } = props.clone();

    html! {
        <BodyText tag="div" class={classes!("p-2", "w-full", "box-border", "bg-paper-danger", class)}>
            <div class={classes!("w-6", "h-6", "inline-block", "mr-1")}>
                <ExclamationCircleIcon class="fill-transparent stroke-danger align-text-bottom"/>
            </div>
            {children}
        </BodyText>
    }
}
