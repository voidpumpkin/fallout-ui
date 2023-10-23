use yew::prelude::*;

#[derive(Clone, Properties, PartialEq)]
pub struct Props {
    #[prop_or_default]
    pub class: Classes,
    #[prop_or_default]
    pub children: Children,
}

#[function_component]
pub fn VerticalDataList(props: &Props) -> Html {
    let Props { class, children } = props.clone();

    html! {
        <table class={classes!("grid", "gap-x-1", "lg:grid-cols-[auto_1fr]", "justify-items-start", class)}>
            {children}
        </table>
    }
}
