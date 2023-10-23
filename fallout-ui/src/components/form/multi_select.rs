use core::fmt::Debug;

use yew::prelude::*;

use crate::components::form::base::base_multi_select::BaseMultiSelect;
use crate::utils::form::FieldControlProps;
use clone_on_capture::clone_on_capture;

#[derive(Clone, PartialEq, Properties)]
pub struct Props<O>
where
    O: Clone + Debug + PartialEq + ToString + 'static,
{
    pub label: String,
    pub field_control_props: FieldControlProps<Vec<O>>,
    pub options: Vec<O>,
    #[prop_or_default]
    pub container_class: Classes,
}

#[clone_on_capture]
#[function_component]
pub fn MultiSelect<O = String>(props: &Props<O>) -> Html
where
    O: Clone + Debug + PartialEq + ToString + 'static,
{
    let Props {
        label,
        field_control_props:
            FieldControlProps {
                value,
                error,
                onchange: on_change,
                onblur,
            },
        options,
        container_class,
    } = props.clone();

    html! {
        <BaseMultiSelect<O>
            {label}
            {container_class}
            {options}
            {on_change}
            {value}
            {error}
            {onblur}
        />
    }
}
