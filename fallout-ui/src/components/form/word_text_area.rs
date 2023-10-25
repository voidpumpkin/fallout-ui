use yew::prelude::*;

use crate::components::form::base::base_text_area::BaseTextArea;
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
    pub tip: Option<String>,
}

#[function_component]
pub fn WordTextArea(props: &Props) -> Html {
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
        tip,
    } = props.clone();

    let handle_input = {
        let onchange = onchange.clone();
        Callback::from(move |(value, _): (String, _)| {
            let value = value.trim_start().into();
            onchange.emit(value);
        })
    };

    let handle_change = {
        Callback::from(move |(value, _): (String, _)| {
            let value = value.trim().into();
            onchange.emit(value);
        })
    };

    html! {
        <BaseTextArea
            {label}
            {value}
            oninput={handle_input}
            onchange={handle_change}
            {tip}
            {onblur}
            {required}
            {disabled}
            {error}
        />
    }
}
