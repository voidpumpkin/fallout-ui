use yew::prelude::*;

use crate::components::form::base::base_filterable_nullable_select::BaseFilterableNullableSelect;
use crate::utils::form::FieldControlProps;

#[derive(Clone, PartialEq, Properties)]
pub struct Props<O: Clone + core::fmt::Debug + PartialEq + ToString + 'static> {
    pub label: String,
    pub field_control_props: FieldControlProps<Option<O>>,
    pub options: Vec<O>,
    #[prop_or_default]
    pub tip: Option<String>,
    #[prop_or_default]
    pub required: bool,
    #[prop_or_default]
    pub disabled: bool,
    #[prop_or_default]
    pub class: Classes,
    #[prop_or_default]
    pub decoration_right: Html,
    #[prop_or_default]
    pub label_key: Option<String>,
}

#[function_component]
pub fn FilterableNullableSelect<
    O: Clone + core::fmt::Debug + PartialEq + ToString + 'static = String,
>(
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
        tip,
        required,
        disabled,
        class,
        decoration_right,
        label_key,
    } = props.clone();

    html! {
        <BaseFilterableNullableSelect<O>
            {label_key}
            {label}
            options={options}
            {value}
            on_select={onchange}
            {tip}
            {error}
            {onblur}
            {required}
            {disabled}
            container_class={class}
            {decoration_right}
        />
    }
}
