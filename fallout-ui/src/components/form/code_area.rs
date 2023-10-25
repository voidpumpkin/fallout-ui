use yew::prelude::*;

use crate::components::form::base::base_code_area::BaseCodeArea;
use crate::utils::form::FieldControlProps;

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    pub label: String,
    pub field_control_props: FieldControlProps<String>,
    #[prop_or_default]
    pub min_lines: usize,
    #[prop_or_default]
    pub textarea_class: Classes,
    #[prop_or_default]
    pub data_qa: String,
}

#[function_component]
pub fn CodeArea(props: &Props) -> Html {
    let Props {
        label,
        field_control_props:
            FieldControlProps {
                value,
                error,
                onchange,
                onblur,
            },
        textarea_class,
        min_lines,
        data_qa,
    } = props.clone();

    html! {
        <BaseCodeArea
            {label}
            {value}
            {error}
            oninput={onchange.reform(|(value,_)| value)}
            {onblur}
            {textarea_class}
            {min_lines}
            {data_qa}
        />
    }
}
