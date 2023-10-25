use clone_on_capture::clone_on_capture;
use gloo::file::File;
use wasm_bindgen::JsValue;
use web_sys::HtmlInputElement;
use yew::prelude::*;

use crate::components::form::base::base_file_input::BaseFileInput;
use crate::utils::dummy_serializable::DummySerializable;
use crate::utils::form::FieldControlProps;
use crate::utils::toasts::notify_err;
use crate::utils::web_error::web_err_js;

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    pub label: String,
    pub field_control_props: FieldControlProps<Option<DummySerializable<File>>>,
    #[prop_or_default]
    pub required: bool,
    #[prop_or_default]
    pub disabled: bool,
    #[prop_or_default]
    pub tip: Option<String>,
}

/// !!! This component does not support value overriding programmatically (It is uncontrolled)
/// But will reset when value changes to None
#[clone_on_capture]
#[function_component]
pub fn FileInput(props: &Props) -> Html {
    let Props {
        label,
        field_control_props:
            FieldControlProps {
                error,
                onchange,
                onblur,
                value,
            },
        tip,
        required,
        disabled,
    } = props.clone();

    let on_change = onchange.reform(|file: Option<File>| file.map(DummySerializable::from));

    let input_ref = use_node_ref();

    use_effect_with(value.clone(), move |value: &Option<_>| {
        if value.is_none() {
            if let Some(input_element) = input_ref.cast::<HtmlInputElement>() {
                if let Err(err) =
                    js_sys::Reflect::set(&input_element, &"value".into(), &JsValue::NULL)
                {
                    notify_err(web_err_js(err));
                }
            }
        }
        || {}
    });

    html! {
        <BaseFileInput
            file_name={value.map(|v| v.0.name())}
            on_blur={onblur}
            {on_change}
            {error}
            {required}
            {disabled}
            {tip}
            {label}
            {input_ref}
        />
    }
}
