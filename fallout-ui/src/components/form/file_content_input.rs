use gloo::file::File;
use gloo::file::FileReadError;
use serde::Deserialize;
use serde::Serialize;
use wasm_bindgen::JsValue;
use web_sys::HtmlInputElement;
use yew::prelude::*;

use crate::components::form::base::base_file_input::BaseFileInput;
use crate::utils::form::FieldControlProps;
use crate::utils::toasts::notify_err;
use crate::utils::upload_file;
use crate::utils::web_error::web_err_js;
use crate::utils::web_error::web_err_logic;

#[derive(Clone, PartialEq, Eq, Debug, Default, Serialize, Deserialize)]
pub struct FileContent {
    pub file_name: Option<String>,
    pub content: Option<String>,
    pub loading: bool,
}

impl FileContent {
    pub fn new(file_name: Option<String>, content: Option<String>) -> Self {
        Self {
            file_name,
            content,
            loading: false,
        }
    }
}

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    pub label: String,
    pub field_control_props: FieldControlProps<FileContent>,
    #[prop_or_default]
    pub required: bool,
    #[prop_or_default]
    pub disabled: bool,
    #[prop_or_default]
    pub tip: Option<String>,
}

#[function_component]
pub fn FileContentInput(props: &Props) -> Html {
    let Props {
        label,
        field_control_props:
            FieldControlProps {
                error,
                onchange,
                onblur,
                value,
            },
        required,
        disabled,
        tip,
    } = props.clone();

    let file_reader_handle = use_state(|| None);
    let disabled = file_reader_handle.is_some() || disabled;

    let input_ref = use_node_ref();

    let upload_callback = {
        let value = value.clone();
        let onchange = onchange.clone();
        let file_reader_handle = file_reader_handle.clone();
        Callback::from(move |result: Result<String, FileReadError>| {
            let mut file_name = value.file_name.clone();
            let content = match result {
                Ok(value) => Some(value),
                Err(err) => {
                    file_name = None;
                    notify_err(web_err_logic(err));
                    None
                }
            };
            file_reader_handle.set(None);

            onchange.emit(FileContent {
                file_name,
                content,
                loading: false,
            });
        })
    };

    let on_change = {
        let file_reader_handle = file_reader_handle.clone();
        let onchange = onchange.clone();
        Callback::from(move |file: Option<File>| {
            let data = match file {
                None => FileContent::default(),
                Some(file) => FileContent {
                    file_name: Some(file.name()),
                    content: None,
                    loading: false,
                },
            };
            file_reader_handle.set(None);
            onchange.emit(data);
        })
    };

    use_effect_with(value.file_name.clone(), {
        let input_ref = input_ref.clone();
        let value = value.clone();
        move |_| {
            if value.file_name.is_some() {
                onchange.emit(FileContent {
                    loading: true,
                    ..value.clone()
                });
                match upload_file(&input_ref, upload_callback) {
                    Ok(reader) => {
                        file_reader_handle.set(Some(reader));
                    }
                    Err(err) => {
                        onchange.emit(FileContent {
                            loading: false,
                            file_name: None,
                            ..value.clone()
                        });
                        notify_err(err);
                        file_reader_handle.set(None);
                    }
                }
            } else if let Some(input_element) = input_ref.cast::<HtmlInputElement>() {
                if let Err(err) =
                    js_sys::Reflect::set(&input_element, &"value".into(), &JsValue::NULL)
                {
                    notify_err(web_err_js(err));
                }
            }
            || {}
        }
    });

    let file_name = value.file_name;

    html! {
        <>
            <BaseFileInput
                {input_ref}
                {file_name}
                on_blur={onblur}
                {on_change}
                {error}
                {required}
                {disabled}
                {tip}
                {label}
            />
        </>
    }
}
