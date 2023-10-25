use yew::prelude::*;

use crate::components::form::base::base_checkbox_input::BaseCheckboxInput;
use crate::utils::form::FieldControlProps;

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    pub label: String,
    pub field_control_props: FieldControlProps<bool>,
    #[prop_or_default]
    pub disabled: bool,
    #[prop_or_default]
    pub label_class: Classes,
}

#[function_component]
pub fn CheckboxInput(props: &Props) -> Html {
    let Props {
        label,
        field_control_props: FieldControlProps {
            value, onchange, ..
        },
        disabled,
        label_class,
    } = props.clone();

    let onchange = onchange.reform(move |(state_change, _)| state_change);

    html! {
        <BaseCheckboxInput {label} {label_class} checked={value} {onchange} {disabled}/>
    }
}
