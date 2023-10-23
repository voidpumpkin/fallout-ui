use yew::prelude::*;

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    #[prop_or(String::from("h1"))]
    pub tag: String,
    #[prop_or(String::from("text-secondary"))]
    pub color_class: String,
    #[prop_or_default]
    pub children: Children,
}

#[function_component]
pub fn PageHeader(props: &Props) -> Html {
    let Props {
        tag,
        children,
        color_class,
    } = props.clone();

    let class = classes!(
        "font-sans",
        "m-0",
        "text-3xl",
        "lg:text-4xl",
        "font-medium",
        "lg:mb-4",
        color_class
    );

    html! {
        <@{tag} {class}>
            {children}
        </@>
    }
}
