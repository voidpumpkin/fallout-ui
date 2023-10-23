use std::collections::BTreeMap;

use yew::prelude::*;

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    pub legend: String,
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub data_qa: String,
    #[prop_or_default]
    pub error: Option<String>,
    #[prop_or_default]
    pub compact: bool,
    #[prop_or_default]
    pub container_class: Classes,
    #[prop_or(classes!("text-secondary"))]
    pub legend_class: Classes,
    #[prop_or_default]
    pub inputs_container_class: Option<Classes>,
    #[prop_or_default]
    pub tip: Option<String>,
    #[prop_or_default]
    pub dyn_attributes: BTreeMap<&'static str, AttrValue>,
}

#[function_component]
pub fn Fieldset(props: &Props) -> Html {
    let Props {
        children,
        data_qa,
        error,
        legend,
        compact,
        container_class,
        legend_class,
        inputs_container_class,
        tip,
        dyn_attributes,
    } = props.clone();

    let legend_class = classes!(
        "font-sans",
        "m-0",
        "text-xl",
        "font-normal",
        if error.is_some() {
            classes!("text-danger")
        } else {
            legend_class
        },
    );

    let inputs_container_class = inputs_container_class
        .unwrap_or_else(|| classes!("flex", "flex-col", "w-full", (!compact).then_some("gap-4")));
    let line_class = classes!(
        "border-1",
        "border-thirdly",
        "border-solid",
        "self-stretch",
        "ml-1",
        if compact { "mr1" } else { "mr-2" }
    );
    let container_class = classes!("m-0", "p-0", "border-0", container_class);

    let under_legend_text = match error {
        Some(error) if !error.is_empty() => {
            html! {<div class="font-sans font-normal text-sm text-danger first-letter:uppercase ml-1">{error}</div>}
        }
        None | Some(_) => html! {
            if let Some(tip) = tip {
                <small class={"font-sans font-normal text-sm text-washed-out-secondary"}>{tip}</small>
            }
        },
    };

    let mut html = html! {
        <fieldset class={container_class} data-qa={format!("{data_qa}-fieldset")} >
            <legend class={legend_class}>{legend}</legend>
            {under_legend_text}
            <div class="flex">
                <div class={line_class} />
                <div class={inputs_container_class}>
                    {for children}
                </div>
            </div>
        </fieldset>
    };

    // The html! macro does not provide a better way to do this
    if let Html::VTag(fieldset_virtual_tag) = &mut html {
        let fieldset_virtual_tag = fieldset_virtual_tag.as_mut();
        dyn_attributes
            .into_iter()
            .for_each(|(key, val)| fieldset_virtual_tag.add_attribute(key, val));
    }

    html
}
