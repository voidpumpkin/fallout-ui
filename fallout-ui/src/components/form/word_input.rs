use yew::prelude::*;

use crate::components::form::text_input::TextInput;
use crate::utils::form::FieldControlProps;

fn transform_input(value: String) -> String {
    value.trim_start().into()
}

fn transform_change(value: String) -> String {
    value.trim().into()
}

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    pub label: String,
    pub field_control_props: FieldControlProps<String>,
    #[prop_or_default]
    pub disabled: bool,
    #[prop_or_default]
    pub required: bool,
    #[prop_or_default]
    pub tip: Option<String>,
    #[prop_or_default]
    pub input_ref: NodeRef,
    #[prop_or_default]
    pub container_class: Classes,
}

#[function_component]
pub fn WordsInput(props: &Props) -> Html {
    let Props {
        label,
        field_control_props,
        disabled,
        required,
        tip,
        input_ref,
        container_class,
    } = props.clone();

    html! {
        <TextInput
            {label}
            {field_control_props}
            {input_ref}
            {required}
            {disabled}
            {tip}
            {transform_input}
            {transform_change}
            {container_class}
        />
    }
}
