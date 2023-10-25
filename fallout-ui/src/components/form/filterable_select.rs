use clone_on_capture::clone_on_capture;
use yew::prelude::*;

use crate::components::form::base::base_filterable_nullable_select::BaseFilterableNullableSelect;
use crate::utils::form::FieldControlProps;

#[derive(Clone, PartialEq, Properties)]
pub struct Props<O: Clone + core::fmt::Debug + PartialEq + ToString + 'static> {
    pub label: String,
    pub field_control_props: FieldControlProps<O>,
    pub options: Vec<O>,
    #[prop_or_default]
    pub required: bool,
    #[prop_or_default]
    pub disabled: bool,
    #[prop_or_default]
    pub container_class: Classes,
}

#[clone_on_capture]
#[function_component]
pub fn FilterableSelect<O: Clone + core::fmt::Debug + PartialEq + ToString + 'static = String>(
    props: &Props<O>,
) -> Html {
    let Props {
        label,
        field_control_props:
            FieldControlProps {
                value,
                error,
                onchange,
                onblur,
            },
        options,
        required,
        disabled,
        container_class,
    } = props.clone();

    let key_handle = use_state(usize::default);
    let key = *key_handle;

    let on_select = move |value| match value {
        Some(value) => onchange.emit(value),
        None => key_handle.set(*key_handle + 1),
    };

    html! {
        <BaseFilterableNullableSelect<O>
            {key}
            {label}
            options={options}
            value={Some(value)}
            {on_select}
            {error}
            {onblur}
            {required}
            {disabled}
            {container_class}
        />
    }
}
