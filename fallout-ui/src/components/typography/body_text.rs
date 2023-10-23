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
    #[prop_or_default]
    pub data_qa: Option<String>,
}

#[function_component]
pub fn BodyText(props: &Props) -> Html {
    let Props {
        tag,
        class,
        children,
        color_class,
        data_qa,
    } = props.clone();

    let class = classes!(
        classes!("font-sans", "m-0", "text-base"),
        color_class,
        class
    );

    html! {
        <@{tag} {class} {data_qa}>
            {children}
        </@>
    }
}
