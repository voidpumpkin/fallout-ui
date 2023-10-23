use yew::prelude::*;

#[derive(Clone, PartialEq, Eq, Properties)]
pub struct Props {
    #[prop_or(classes!("border-black"))]
    pub class: Classes,
}

#[function_component]
pub fn Divider(props: &Props) -> Html {
    let Props { class } = props.clone();
    html! {
        <div class={classes!("w-full", "border-1", "border-solid", class)} />
    }
}
