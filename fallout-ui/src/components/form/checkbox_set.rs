use std::collections::HashSet;

use yew::prelude::*;

use crate::components::form::base::base_checkbox_set::BaseCheckboxSet;
use crate::utils::form::FieldControlProps;

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    pub legend: String,
    pub field_control_props: FieldControlProps<HashSet<String>>,
    #[prop_or_default]
    pub required: bool,
    #[prop_or_default]
    pub available: Vec<String>,
    #[prop_or_default]
    pub disabled: Vec<String>,
    #[prop_or_default]
    pub data_qa: String,
}

#[function_component]
pub fn CheckboxSet(props: &Props) -> Html {
    let Props {
        legend,
        field_control_props:
            FieldControlProps {
                value,
                error,
                onchange,
                ..
            },
        required,
        available,
        disabled,
        data_qa,
    } = props.clone();

    let checked = value.into_iter().collect::<Vec<_>>();

    html! {
        <BaseCheckboxSet {legend} {required} {available} {checked} {disabled} {data_qa} {error} {onchange}/>
    }
}
