use yew::prelude::*;
use yew_heroicons::size_24::outline::ExclamationTriangleIcon;

use crate::components::typography::body_text::BodyText;

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    pub children: Children,
    #[prop_or_default]
    pub class: Classes,
}

#[function_component]
pub fn CalloutWarning(props: &Props) -> Html {
    let Props { children, class } = props.clone();

    html! {
        <BodyText tag="div" class={classes!("p-2", "w-full", "box-border", "bg-paper-primary", class)}>
            <div class={classes!("w-6", "h-6", "inline-block", "mr-1")}>
                <ExclamationTriangleIcon class="fill-transparent stroke-primary align-text-bottom"/>
            </div>
            {children}
        </BodyText>
    }
}
