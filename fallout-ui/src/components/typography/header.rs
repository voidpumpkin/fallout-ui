use yew::prelude::*;

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    #[prop_or(String::from("h2"))]
    pub tag: String,
    #[prop_or(String::from("text-secondary"))]
    pub color_class: String,
    #[prop_or_default]
    pub class: Classes,
    pub children: Children,
}

#[function_component]
pub fn Header(props: &Props) -> Html {
    let Props {
        tag,
        class,
        children,
        color_class,
    } = props.clone();

    let class = classes!(
        classes!("font-sans", "m-0", "text-2xl", "font-normal"),
        color_class,
        class
    );

    html! {
        <@{tag} {class}>
            {children}
        </@>
    }
}
