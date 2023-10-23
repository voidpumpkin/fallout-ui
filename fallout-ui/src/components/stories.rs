use yew::prelude::*;

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    #[prop_or_default]
    pub class: Classes,
    #[prop_or_default]
    pub children: Children,
}

#[function_component]
pub fn Stories(props: &Props) -> Html {
    let Props { children, class } = props.clone();
    html! {
        <div class={classes!("flex","gap-5","flex-col",class)}>
            {children}
        </div>
    }
}
