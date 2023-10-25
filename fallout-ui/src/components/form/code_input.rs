use clone_on_capture::clone_on_capture;
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
    #[prop_or_default]
    pub readonly: bool,
    #[prop_or(String::from("text"))]
    pub input_mode: String,
    #[prop_or_default]
    pub tip: Option<String>,
    #[prop_or_default]
    pub input_ref: NodeRef,
    #[prop_or_default]
    pub onfocus: Callback<()>,
    #[prop_or_default]
    pub decoration_right: Html,
    /// Used for cases when you need non-unique looking labels visually, but unique semantically
    /// e.g. instead of Director 1, Director 2, its just Director, Director
    #[prop_or_default]
    pub label_key: Option<String>,
}

#[clone_on_capture]
#[function_component]
pub fn CodeInput(props: &Props) -> Html {
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
        readonly,
        input_mode,
        tip,
        input_ref,
        onfocus,
        label_key,
        decoration_right,
    } = props.clone();

    let handle_input = move |(value, _): (String, _)| onchange.emit(value.trim().into());

    let handle_change = move |(value, _): (String, _)| onchange.emit(value.trim_start().into());

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
            {readonly}
            {input_ref}
            {onfocus}
            input_type="text"
            {input_mode}
            {error}
            input_class={"font-mono"}
            {decoration_right}
        />
    }
}
