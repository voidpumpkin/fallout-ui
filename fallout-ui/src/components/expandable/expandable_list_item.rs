use yew::prelude::*;

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    #[prop_or_default]
    pub children: Children,
}

#[function_component]
pub fn ExpandableListItem(props: &Props) -> Html {
    let Props { children } = props.clone();
    html! {<li>{children}</li>}
}
