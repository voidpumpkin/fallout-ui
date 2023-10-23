use yew::prelude::*;

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    pub on_submit: Callback<SubmitEvent>,
    #[prop_or_default]
    pub id: String,
    #[prop_or(classes!("flex", "flex-col", "gap-4", "lg:max-w-md"))]
    pub class: Classes,
    pub children: Children,
}

#[function_component]
pub fn Form(props: &Props) -> Html {
    let Props {
        on_submit,
        id,
        children,
        class,
    } = props.clone();

    let onsubmit = on_submit.reform(move |e: SubmitEvent| {
        e.prevent_default();
        e
    });

    html! {
        <form {id} {class} {onsubmit} novalidate={true}>
            {for children}
        </form>
    }
}
