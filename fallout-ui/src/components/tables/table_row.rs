use yew::prelude::*;

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    #[prop_or_default]
    pub onclick: Option<Callback<MouseEvent>>,
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub data_qa: String,
}

#[function_component]
pub fn TableRow(props: &Props) -> Html {
    let Props {
        children,
        onclick,
        data_qa,
    } = props.clone();

    let class = classes!(
        "bg-washed-out-thirdly",
        "even:bg-white",
        "border-0",
        "border-b",
        "border-b-secondary",
        "border-solid",
        "transition",
        "duration-300",
        "ease-in-out",
        onclick
            .is_some()
            .then(|| classes!("hover:bg-thirdly", "hover:cursor-pointer"))
    );

    let data_qa = format!("{data_qa}-item");

    html! {
        <tr {class} {onclick} data-qa={data_qa}>
            {children}
        </tr>
    }
}
