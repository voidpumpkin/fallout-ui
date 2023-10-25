use yew::prelude::*;

use crate::components::form::base::base_input::BaseInput;
use crate::utils::form::FieldControlProps;

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    pub label: String,
    pub field_control_props: FieldControlProps<String>,
    #[prop_or_default]
    pub required: bool,
    #[prop_or_default]
    pub disabled: bool,
    #[prop_or(String::from("text"))]
    pub input_mode: String,
    #[prop_or_default]
    pub tip: Option<String>,
    #[prop_or_default]
    pub transform_input: Option<Callback<String, String>>,
    #[prop_or_default]
    pub transform_change: Option<Callback<String, String>>,
    #[prop_or_default]
    pub input_ref: NodeRef,
    #[prop_or_default]
    pub onfocus: Callback<()>,
    /// Used for cases when you need non-unique looking labels visually, but unique semantically
    /// e.g. instead of Director 1, Director 2, its just Director, Director
    #[prop_or_default]
    pub label_key: Option<String>,
    #[prop_or_default]
    pub container_class: Classes,
    #[prop_or_default]
    pub decoration_left: Html,
    #[prop_or_default]
    pub decoration_right: Html,
}

#[function_component]
pub fn TextInput(props: &Props) -> Html {
    let Props {
        label,
        field_control_props:
            FieldControlProps {
                value,
                error,
                onchange,
                onblur,
            },
        required,
        disabled,
        input_mode,
        tip,
        transform_input,
        transform_change,
        input_ref,
        onfocus,
        label_key,
        container_class,
        decoration_left,
        decoration_right,
    } = props.clone();

    let handle_input = {
        let onchange = onchange.clone();
        Callback::from(move |(value, _): (String, _)| {
            let value = match transform_input.clone() {
                None => value,
                Some(transform) => transform.emit(value),
            };
            onchange.emit(value);
        })
    };

    let handle_change = {
        Callback::from(move |(value, _): (String, _)| {
            let value = match transform_change.clone() {
                None => value,
                Some(transform) => transform.emit(value),
            };
            onchange.emit(value);
        })
    };

    html! {
        <BaseInput
            {label}
            {label_key}
            {value}
            oninput={handle_input}
            onchange={handle_change}
            {tip}
            {onblur}
            {required}
            {disabled}
            {input_ref}
            {onfocus}
            input_type="text"
            {input_mode}
            {error}
            {container_class}
            {decoration_left}
            {decoration_right}
        />
    }
}
