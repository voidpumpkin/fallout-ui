use yew::prelude::*;

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    #[prop_or_default]
    pub wrap: bool,
    #[prop_or(String::from("code"))]
    pub tag: String,
    #[prop_or(String::from("text-secondary"))]
    pub color_class: String,
    #[prop_or_default]
    pub class: Classes,
    #[prop_or_default]
    pub children: Children,
}

#[function_component]
pub fn Code(props: &Props) -> Html {
    let Props {
        wrap,
        tag,
        class,
        children,
        color_class,
    } = props.clone();

    let class = classes!(
        classes!(
            "block",
            "font-mono",
            "m-0",
            "p-1",
            "text-sm",
            "border-4",
            "rounded-lg",
            "border-white",
            "bg-white"
        ),
        wrap.then_some("whitespace-pre-wrap"),
        color_class,
        class
    );

    html! {
        <@{tag} {class}>
            {children}
        </@>
    }
}
