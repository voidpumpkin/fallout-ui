use yew::prelude::*;

use crate::components::form::text_input::TextInput;
use crate::utils::form::FieldControlProps;

fn transform_input(value: String) -> String {
    value
        .trim_start()
        .chars()
        .enumerate()
        .filter_map(|(i, c)| {
            if i == 0 {
                (c.is_numeric() || c == '+').then_some(c)
            } else {
                c.is_numeric().then_some(c)
            }
        })
        .collect::<String>()
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
pub fn PhoneInput(props: &Props) -> Html {
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
        <TextInput {container_class} {label} {field_control_props} {input_ref} {disabled} {required} {tip} {transform_input} {transform_change} input_mode={"tel"}/>
    }
}
