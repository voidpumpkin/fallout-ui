use yew::prelude::*;

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    #[prop_or(String::from("p"))]
    pub tag: String,
    #[prop_or(String::from("text-secondary"))]
    pub color_class: String,
    #[prop_or_default]
    pub class: Classes,
    #[prop_or_default]
    pub children: Children,
}

#[function_component]
pub fn SmallBodyText(props: &Props) -> Html {
    let Props {
        tag,
        class,
        children,
        color_class,
    } = props.clone();

    let class = classes!(classes!("font-sans", "m-0", "text-sm"), color_class, class);

    html! {
        <@{tag} {class}>
            {children}
        </@>
    }
}
