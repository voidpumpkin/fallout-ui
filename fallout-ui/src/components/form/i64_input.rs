use yew::prelude::*;

use crate::components::form::base::base_input::BaseInput;
use crate::utils::form::FieldControlProps;

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
}

#[function_component]
pub fn I64Input(props: &Props) -> Html {
    let Props {
        label,
        field_control_props:
            FieldControlProps {
                value,
                error,
                onchange,
                onblur,
            },
        disabled,
        required,
        tip,
        input_ref,
    } = props.clone();

    let handle_input = {
        let onchange = onchange.clone();
        Callback::from(move |(value, _): (String, _)| {
            let value = value
                .trim_start()
                .chars()
                .filter(|c| c.is_numeric())
                .collect::<String>();
            onchange.emit(value);
        })
    };

    let handle_change = {
        Callback::from(move |(value, _): (String, _)| {
            onchange.emit(value.trim().into());
        })
    };

    html! {
        <BaseInput
            {label}
            {value}
            oninput={handle_input}
            onchange={handle_change}
            {tip}
            {onblur}
            {required}
            {disabled}
            {input_ref}
            input_mode="numeric"
            {error}
        />
    }
}
